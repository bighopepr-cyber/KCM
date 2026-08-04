#!/usr/bin/env python3
"""
KCM Benchmark Regression Detector

Compares current benchmark results against a baseline and reports regressions.
Usage:
    python3 bench-regression.py --baseline baseline.json --current current.json
    python3 bench-regression.py --baseline baseline.json --current .
"""

import json
import sys
import argparse
from pathlib import Path

WARN_THRESHOLD = 0.05   # 5% regression triggers warning
FAIL_THRESHOLD = 0.10   # 10% regression triggers failure

def load_results(path):
    results = {}
    p = Path(path)
    if p.is_dir():
        for f in p.glob("**/new/*.json"):
            try:
                with open(f) as fh:
                    data = json.load(fh)
                    name = data.get("function_id", f.stem)
                    mean = data.get("mean", {}).get("point_estimate", 0)
                    results[name] = mean
            except (json.JSONDecodeError, KeyError):
                pass
    elif p.is_file():
        with open(p) as fh:
            data = json.load(fh)
            for entry in data:
                name = entry.get("function_id", "unknown")
                mean = entry.get("mean", {}).get("point_estimate", 0)
                results[name] = mean
    return results

def compare(baseline, current):
    warnings = 0
    failures = 0
    
    print(f"{'Benchmark':<50} {'Baseline':>12} {'Current':>12} {'Change':>10} {'Status':>8}")
    print("-" * 100)
    
    for name in sorted(set(baseline.keys()) & set(current.keys())):
        base = baseline[name]
        curr = current[name]
        if base == 0:
            continue
        change = (curr - base) / base
        
        if change > FAIL_THRESHOLD:
            status = "FAIL"
            failures += 1
        elif change > WARN_THRESHOLD:
            status = "WARN"
            warnings += 1
        else:
            status = "OK"
        
        print(f"{name:<50} {base:>12.0f} {curr:>12.0f} {change:>+9.1%} {status:>8}")
    
    only_in_current = set(current.keys()) - set(baseline.keys())
    for name in sorted(only_in_current):
        print(f"{name:<50} {'N/A':>12} {current[name]:>12.0f} {'NEW':>10} {'ADD':>8}")
    
    only_in_baseline = set(baseline.keys()) - set(current.keys())
    for name in sorted(only_in_baseline):
        print(f"{name:<50} {baseline[name]:>12.0f} {'N/A':>12} {'GONE':>10} {'DEL':>8}")
    
    print()
    print(f"Summary: {len(baseline)} baseline, {len(current)} current")
    print(f"  Regressions: {failures} failures, {warnings} warnings")
    
    if failures > 0:
        print(f"\nRESULT: FAIL ({failures} benchmarks regressed >{FAIL_THRESHOLD:.0%})")
        return 1
    elif warnings > 0:
        print(f"\nRESULT: WARN ({warnings} benchmarks regressed >{WARN_THRESHOLD:.0%})")
        return 0
    else:
        print(f"\nRESULT: PASS")
        return 0

def main():
    parser = argparse.ArgumentParser(description="KCM Benchmark Regression Detector")
    parser.add_argument("--baseline", required=True, help="Baseline results path")
    parser.add_argument("--current", required=True, help="Current results path")
    args = parser.parse_args()
    
    baseline = load_results(args.baseline)
    current = load_results(args.current)
    
    if not baseline:
        print("ERROR: No baseline results found")
        return 1
    if not current:
        print("ERROR: No current results found")
        return 1
    
    return compare(baseline, current)

if __name__ == "__main__":
    sys.exit(main())
