#!/usr/bin/env python3
"""
KCM Benchmark Regression Detector

Compares current benchmark results against a stored baseline.
Generates JSON, CSV, and Markdown reports.
Exits with code 1 if any regression exceeds the configured threshold.

Usage:
    python3 tools/bench-compare.py [--threshold-warn 5] [--threshold-fail 10] [--baseline-path benchmark-results/baseline.json]

Exit codes:
    0 = No regressions above threshold
    1 = Regressions detected above fail threshold
    2 = No baseline exists (first run — creates baseline)
"""

import argparse
import json
import os
import re
import sys
from datetime import datetime, timezone


def parse_bench_log(log_path):
    """Parse Criterion benchmark output and extract results."""
    results = {}
    with open(log_path) as f:
        content = f.read()

    # Match Criterion output lines: "name time: [lower mid upper]"
    pattern = r'(\S+)\s+time:\s+\[([0-9.]+)\s+(\S+)\s+([0-9.]+)\s+\S+\s+([0-9.]+)\s+\S+\]'
    for match in re.finditer(pattern, content):
        name = match.group(1)
        lower = float(match.group(2))
        unit_str = match.group(3)
        median = float(match.group(4))
        upper = float(match.group(5))

        # Determine unit from captured group
        if 'ns' in unit_str:
            unit = 'ns'
        elif 'µs' in unit_str or 'us' in unit_str:
            unit = 'us'
        elif 'ms' in unit_str:
            unit = 'ms'
        else:
            unit = 'ns'

        # Convert to nanoseconds
        multiplier = {'ns': 1, 'us': 1000, 'ms': 1_000_000}.get(unit, 1)

        results[name] = {
            'lower_ns': lower * multiplier,
            'median_ns': median * multiplier,
            'upper_ns': upper * multiplier,
            'unit': unit,
        }

    return results


def load_baseline(path):
    """Load baseline JSON file."""
    if not os.path.exists(path):
        return None
    with open(path) as f:
        return json.load(f)


def save_baseline(path, results, environment):
    """Save current results as baseline."""
    baseline = {
        'version': '2.0',
        'created': datetime.now(timezone.utc).isoformat(),
        'environment': environment,
        'benchmarks': results,
    }
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w') as f:
        json.dump(baseline, f, indent=2)


def compare(baseline_benchmarks, current_benchmarks, warn_pct, fail_pct):
    """Compare baseline against current results. Returns list of alerts."""
    alerts = []
    warn_threshold = warn_pct / 100.0
    fail_threshold = fail_pct / 100.0

    for name, current in current_benchmarks.items():
        if name not in baseline_benchmarks:
            alerts.append({
                'name': name,
                'type': 'new_benchmark',
                'severity': 'info',
                'message': f'New benchmark (no baseline)',
            })
            continue

        baseline = baseline_benchmarks[name]
        base_median = baseline['median_ns']
        curr_median = current['median_ns']

        if base_median == 0:
            continue

        # Regression = current is SLOWER than baseline (higher ns)
        change_pct = ((curr_median - base_median) / base_median) * 100.0

        if change_pct > fail_threshold:
            alerts.append({
                'name': name,
                'type': 'regression',
                'severity': 'critical',
                'baseline_ns': base_median,
                'current_ns': curr_median,
                'change_pct': change_pct,
                'message': f'REGRESSION: {name} slowed by {change_pct:+.1f}% ({base_median/1e6:.2f}ms -> {curr_median/1e6:.2f}ms)',
            })
        elif change_pct > warn_threshold:
            alerts.append({
                'name': name,
                'type': 'regression',
                'severity': 'warning',
                'baseline_ns': base_median,
                'current_ns': curr_median,
                'change_pct': change_pct,
                'message': f'WARNING: {name} slowed by {change_pct:+.1f}% ({base_median/1e6:.2f}ms -> {curr_median/1e6:.2f}ms)',
            })

    for name in baseline_benchmarks:
        if name not in current_benchmarks:
            alerts.append({
                'name': name,
                'type': 'missing_benchmark',
                'severity': 'warning',
                'message': f'Missing benchmark: {name}',
            })

    return alerts


def generate_reports(results, alerts, environment, output_dir):
    """Generate Markdown, JSON, and CSV reports."""
    os.makedirs(output_dir, exist_ok=True)

    # --- Markdown Report ---
    md_lines = [
        '# KCM Performance Benchmark Report',
        '',
        f'**Generated**: {datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")}',
        '',
        '## Environment',
        '',
    ]
    for k, v in environment.items():
        md_lines.append(f'- **{k}**: {v}')
    md_lines.extend(['', '## Performance Results', ''])
    md_lines.append('| Benchmark | Median | Lower | Upper | Throughput |')
    md_lines.append('|-----------|--------|-------|-------|------------|')

    for name, r in sorted(results.items()):
        dur = format_duration(r['median_ns'])
        lower = format_duration(r['lower_ns'])
        upper = format_duration(r['upper_ns'])
        thr = f"{1e9/r['median_ns']:.0f} ops/s" if r['median_ns'] > 0 else 'N/A'
        md_lines.append(f'| {name} | {dur} | {lower} | {upper} | {thr} |')

    if alerts:
        md_lines.extend(['', '## Regression Analysis', ''])
        critical = [a for a in alerts if a['severity'] == 'critical']
        warnings = [a for a in alerts if a['severity'] == 'warning']
        info = [a for a in alerts if a['severity'] == 'info']

        if critical:
            md_lines.append(f'### CRITICAL ({len(critical)} regressions)')
            for a in critical:
                md_lines.append(f'- {a["message"]}')
            md_lines.append('')
        if warnings:
            md_lines.append(f'### WARNING ({len(warnings)} issues)')
            for a in warnings:
                md_lines.append(f'- {a["message"]}')
            md_lines.append('')
        if info:
            md_lines.append(f'### INFO ({len(info)} new benchmarks)')
            for a in info:
                md_lines.append(f'- {a["message"]}')
            md_lines.append('')

    with open(os.path.join(output_dir, 'KCM_BENCHMARK_REPORT.md'), 'w') as f:
        f.write('\n'.join(md_lines))

    # --- JSON Report ---
    json_report = {
        'version': '2.0',
        'generated': datetime.now(timezone.utc).isoformat(),
        'environment': environment,
        'results': results,
        'alerts': alerts,
        'summary': {
            'total_benchmarks': len(results),
            'critical_regressions': len([a for a in alerts if a['severity'] == 'critical']),
            'warnings': len([a for a in alerts if a['severity'] == 'warning']),
        },
    }
    with open(os.path.join(output_dir, 'KCM_BENCHMARK_REPORT.json'), 'w') as f:
        json.dump(json_report, f, indent=2)

    # --- CSV Matrix ---
    with open(os.path.join(output_dir, 'KCM_PERFORMANCE_MATRIX.csv'), 'w') as f:
        f.write('benchmark,median_ns,lower_ns,upper_ns,throughput_ops_sec\n')
        for name, r in sorted(results.items()):
            thr = f"{1e9/r['median_ns']:.0f}" if r['median_ns'] > 0 else '0'
            f.write(f'{name},{r["median_ns"]:.0f},{r["lower_ns"]:.0f},{r["upper_ns"]:.0f},{thr}\n')


def format_duration(ns):
    if ns > 1e9:
        return f'{ns/1e9:.2f} s'
    elif ns > 1e6:
        return f'{ns/1e6:.2f} ms'
    elif ns > 1e3:
        return f'{ns/1e3:.2f} µs'
    else:
        return f'{ns:.0f} ns'


def main():
    parser = argparse.ArgumentParser(description='KCM Benchmark Regression Detector')
    parser.add_argument('--threshold-warn', type=float, default=5.0, help='Warning threshold (%%)')
    parser.add_argument('--threshold-fail', type=float, default=10.0, help='Fail threshold (%%)')
    parser.add_argument('--baseline-path', default='benchmark-results/baseline.json', help='Baseline file')
    parser.add_argument('--bench-log', default='benchmark-results/raw/bench.log', help='Benchmark output log')
    parser.add_argument('--output-dir', default='benchmark-results/reports', help='Report output directory')
    parser.add_argument('--save-baseline', action='store_true', help='Save current results as new baseline')
    parser.add_argument('--update-baseline', action='store_true', help='Update baseline even if regressions found')
    args = parser.parse_args()

    if not os.path.exists(args.bench_log):
        print(f'ERROR: Benchmark log not found at {args.bench_log}', file=sys.stderr)
        print('Run `cargo bench --workspace 2>&1 | tee benchmark-results/raw/bench.log` first.', file=sys.stderr)
        sys.exit(2)

    # Collect environment metadata
    environment = {}
    env_path = 'benchmark-results/metadata/environment.json'
    if os.path.exists(env_path):
        with open(env_path) as f:
            environment = json.load(f)

    # Parse benchmark results
    results = parse_bench_log(args.bench_log)
    if not results:
        print('WARNING: No benchmark results parsed from log. Check log format.', file=sys.stderr)
        sys.exit(2)

    print(f'Parsed {len(results)} benchmark results.')

    # Load or create baseline
    baseline = load_baseline(args.baseline_path)
    if baseline is None:
        print('No baseline found. Creating initial baseline.')
        save_baseline(args.baseline_path, results, environment)
        generate_reports(results, [], environment, args.output_dir)
        print(f'Baseline saved to {args.baseline_path}')
        print('Run again after the baseline is established to detect regressions.')
        sys.exit(0)

    # Compare against baseline
    alerts = compare(baseline.get('benchmarks', {}), results, args.threshold_warn, args.threshold_fail)

    # Generate reports
    generate_reports(results, alerts, environment, args.output_dir)

    # Print summary
    critical = [a for a in alerts if a['severity'] == 'critical']
    warnings = [a for a in alerts if a['severity'] == 'warning']

    print(f'\nRegression Analysis:')
    print(f'  Critical: {len(critical)}')
    print(f'  Warnings: {len(warnings)}')

    if critical:
        print('\n--- CRITICAL REGRESSIONS ---')
        for a in critical:
            print(f'  {a["message"]}')
        print('---')

    if warnings:
        print('\n--- WARNINGS ---')
        for a in warnings:
            print(f'  {a["message"]}')
        print('---')

    # Save updated baseline if requested
    if args.save_baseline or args.update_baseline:
        save_baseline(args.baseline_path, results, environment)
        print(f'\nBaseline updated: {args.baseline_path}')

    # Report paths
    print(f'\nReports generated in {args.output_dir}/:')
    for f in os.listdir(args.output_dir):
        if f.endswith(('.md', '.json', '.csv')):
            print(f'  {f}')

    if critical:
        print(f'\nFAIL: {len(critical)} critical regressions exceed {args.threshold_fail}% threshold.')
        sys.exit(1)
    else:
        print('\nPASS: No critical regressions detected.')
        sys.exit(0)


if __name__ == '__main__':
    main()
