#!/bin/bash
set -euo pipefail

echo "=== KCM Benchmark Report Generator ==="
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

RESULTS_DIR="benchmark-results"
mkdir -p "${RESULTS_DIR}/reports" "${RESULTS_DIR}/raw" "${RESULTS_DIR}/metadata"

echo "Step 1: Collecting environment metadata..."
cat > "${RESULTS_DIR}/metadata/environment.json" << EOF
{
  "os": "$(uname -s) $(uname -r)",
  "cpu": "$(lscpu 2>/dev/null | grep 'Model name' | cut -d: -f2 | xargs || echo 'unknown')",
  "cores": $(nproc 2>/dev/null || echo 1),
  "ram_mb": $(free -m 2>/dev/null | awk '/^Mem:/{print $2}' || echo 0),
  "rust_version": "$(rustc --version 2>/dev/null || echo 'unknown')",
  "llvm_version": "$(llc --version 2>/dev/null | head -1 || echo 'unknown')"
}
EOF

cat > "${RESULTS_DIR}/metadata/git.json" << EOF
{
  "commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
  "branch": "$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'unknown')",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "message": "$(git log -1 --pretty=format:'%s' 2>/dev/null || echo 'unknown')"
}
EOF

cat > "${RESULTS_DIR}/metadata/benchmark-version.json" << EOF
{
  "benchmark_version": "1.0.0",
  "spec_version": "PRD-TESTING-2026",
  "criterion_version": "0.5"
}
EOF

echo "Step 2: Building benchmark targets..."
cargo bench --workspace --no-run 2>&1 | tee "${RESULTS_DIR}/raw/build.log"

echo "Step 3: Running benchmarks..."
cargo bench --workspace 2>&1 | tee "${RESULTS_DIR}/raw/bench.log"

echo "Step 4: Copying criterion results..."
if [ -d "target/criterion" ]; then
    cp -r target/criterion "${RESULTS_DIR}/raw/criterion-results"
fi

echo "Step 5: Generating reports..."
python3 -c "
import json, os, re

bench_log = '${RESULTS_DIR}/raw/bench.log'
report_lines = ['# KCM Benchmark Report', '', '## Environment', '']

try:
    with open('${RESULTS_DIR}/metadata/environment.json') as f:
        env = json.load(f)
        report_lines.append(f'- **OS**: {env.get(\"os\", \"unknown\")}')
        report_lines.append(f'- **CPU**: {env.get(\"cpu\", \"unknown\")}')
        report_lines.append(f'- **Cores**: {env.get(\"cores\", \"unknown\")}')
        report_lines.append(f'- **RAM**: {env.get(\"ram_mb\", \"unknown\")} MB')
        report_lines.append(f'- **Rust**: {env.get(\"rust_version\", \"unknown\")}')
except: pass

report_lines.append('')
try:
    with open('${RESULTS_DIR}/metadata/git.json') as f:
        git = json.load(f)
        report_lines.append(f'- **Commit**: {git.get(\"commit\", \"unknown\")}')
        report_lines.append(f'- **Branch**: {git.get(\"branch\", \"unknown\")}')
        report_lines.append(f'- **Timestamp**: {git.get(\"timestamp\", \"unknown\")}')
except: pass

report_lines.extend(['', '## Performance Results', ''])
report_lines.append('| Benchmark | Duration | Throughput |')
report_lines.append('|-----------|----------|------------|')

summary_data = []
try:
    with open(bench_log) as f:
        content = f.read()
        for match in re.finditer(r'(\S+)\s+(\d+\.\d+)\s+ns', content):
            name, ns = match.group(1), float(match.group(2))
            dur = f'{ns/1e9:.2f} s' if ns > 1e9 else f'{ns/1e6:.2f} ms' if ns > 1e6 else f'{ns/1e3:.2f} us' if ns > 1e3 else f'{ns:.0} ns'
            thr = f'{1e9/ns:.0} ops/s' if ns > 0 else 'N/A'
            report_lines.append(f'| {name} | {dur} | {thr} |')
            summary_data.append({'name': name, 'duration_ns': ns, 'throughput_ops_sec': 1e9/ns if ns > 0 else 0})
except: pass

report_lines.extend(['', f'## Summary', '', f'- **Total benchmarks**: {len(summary_data)}', ''])

os.makedirs('${RESULTS_DIR}/reports', exist_ok=True)
with open('${RESULTS_DIR}/reports/KCM_BENCHMARK_REPORT.md', 'w') as f:
    f.write('\n'.join(report_lines))

json_summary = {'benchmark_version': '1.0.0', 'results': summary_data, 'total_benchmarks': len(summary_data)}
with open('${RESULTS_DIR}/reports/KCM_BENCHMARK_SUMMARY.json', 'w') as f:
    json.dump(json_summary, f, indent=2)

with open('${RESULTS_DIR}/reports/KCM_PERFORMANCE_MATRIX.csv', 'w') as f:
    f.write('benchmark,duration_ns,throughput_ops_sec\n')
    for item in summary_data:
        f.write(f'{item[\"name\"]},{item[\"duration_ns\"]},{item[\"throughput_ops_sec\"]:.0}\n')
"

echo ""
echo "=== Reports Generated ==="
ls -la "${RESULTS_DIR}/reports/"
echo ""
echo "=== Done ==="
