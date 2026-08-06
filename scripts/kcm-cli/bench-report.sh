#!/bin/bash
set -euo pipefail

echo "=== KCM Benchmark Report Generator v2.0 ==="
echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

RESULTS_DIR="benchmark-results"
mkdir -p "${RESULTS_DIR}/reports" "${RESULTS_DIR}/raw" "${RESULTS_DIR}/metadata"

echo "Step 1: Collecting environment metadata..."

# System metadata
cat > "${RESULTS_DIR}/metadata/system.json" << EOF
{
  "os": "$(uname -s) $(uname -r)",
  "os_release": "$(cat /etc/os-release 2>/dev/null | grep PRETTY_NAME | cut -d= -f2 | tr -d '"' || echo 'unknown')",
  "hostname": "$(hostname 2>/dev/null || echo 'unknown')",
  "cpu_model": "$(lscpu 2>/dev/null | grep 'Model name' | cut -d: -f2 | xargs || echo 'unknown')",
  "cpu_arch": "$(uname -m)",
  "cpu_cores": $(nproc 2>/dev/null || echo 1),
  "cpu_threads": $(nproc 2>/dev/null || echo 1),
  "ram_bytes": $(free -b 2>/dev/null | awk '/^Mem:/{print $2}' || echo 0),
  "ram_gb": $(free -g 2>/dev/null | awk '/^Mem:/{print $2}' || echo 0),
  "page_size": $(getconf PAGESIZE 2>/dev/null || echo 4096),
  "cpu_cache_l1": "$(lscpu 2>/dev/null | grep 'L1d cache' | cut -d: -f2 | xargs || echo 'unknown')",
  "cpu_cache_l2": "$(lscpu 2>/dev/null | grep 'L2 cache' | cut -d: -f2 | xargs || echo 'unknown')",
  "cpu_cache_l3": "$(lscpu 2>/dev/null | grep 'L3 cache' | cut -d: -f2 | xargs || echo 'unknown')"
}
EOF

# Rust toolchain metadata
cat > "${RESULTS_DIR}/metadata/rust.json" << EOF
{
  "rust_version": "$(rustc --version 2>/dev/null || echo 'unknown')",
  "cargo_version": "$(cargo --version 2>/dev/null || echo 'unknown')",
  "llvm_version": "$(llc --version 2>/dev/null | head -1 || echo 'unknown')",
  "rustup_toolchain": "$(rustup show active-toolchain 2>/dev/null || echo 'unknown')",
  "target_triple": "$(rustc -vV 2>/dev/null | grep host | cut -d: -f2 | xargs || echo 'unknown')",
  "edition": "2024"
}
EOF

# Git metadata
cat > "${RESULTS_DIR}/metadata/git.json" << EOF
{
  "commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
  "commit_short": "$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')",
  "branch": "$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'unknown')",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "message": "$(git log -1 --pretty=format:'%s' 2>/dev/null || echo 'unknown')",
  "author": "$(git log -1 --pretty=format:'%an' 2>/dev/null || echo 'unknown')",
  "dirty": $(git diff --quiet 2>/dev/null && echo "false" || echo "true")
}
EOF

# Build configuration metadata
cat > "${RESULTS_DIR}/metadata/build.json" << EOF
{
  "profile": "release",
  "opt_level": 3,
  "lto": true,
  "codegen_units": 1,
  "strip": true,
  "features": "default",
  "resolver": "3",
  "workspace_members": $(cargo metadata --format-version 1 2>/dev/null | python3 -c "import sys,json; print(len(json.load(sys.stdin)['workspace_members']))" 2>/dev/null || echo 0)
}
EOF

# Benchmark configuration
cat > "${RESULTS_DIR}/metadata/benchmark.json" << EOF
{
  "benchmark_version": "2.0.0",
  "spec_version": "PRD-TESTING-2026",
  "criterion_version": "0.5",
  "measurement_time_secs": 5,
  "warmup_time_secs": 3,
  "sample_size": 100,
  "measurement_time_extended_secs": 10
}
EOF

# Merge all metadata into a single file
python3 -c "
import json, os
meta = {}
for f in ['system.json', 'rust.json', 'git.json', 'build.json', 'benchmark.json']:
    path = os.path.join('${RESULTS_DIR}/metadata', f)
    if os.path.exists(path):
        with open(path) as fh:
            meta.update(json.load(fh))
with open('${RESULTS_DIR}/metadata/environment.json', 'w') as fh:
    json.dump(meta, fh, indent=2)
print('Metadata collected:')
for k, v in meta.items():
    print(f'  {k}: {v}')
"

echo ""
echo "Step 2: Building benchmark targets..."
cargo bench --workspace --no-run 2>&1 | tee "${RESULTS_DIR}/raw/build.log"

echo ""
echo "Step 3: Running benchmarks..."
cargo bench --workspace 2>&1 | tee "${RESULTS_DIR}/raw/bench.log"

echo ""
echo "Step 4: Copying criterion results..."
if [ -d "target/criterion" ]; then
    cp -r target/criterion "${RESULTS_DIR}/raw/criterion-results"
fi

echo ""
echo "Step 5: Generating reports..."
python3 -c "
import json, os, re
from datetime import datetime, timezone

bench_log = '${RESULTS_DIR}/raw/bench.log'
report_lines = ['# KCM Benchmark Report v2.0', '']

# Load metadata
env = {}
env_path = '${RESULTS_DIR}/metadata/environment.json'
if os.path.exists(env_path):
    with open(env_path) as f:
        env = json.load(f)

report_lines.append('## Environment')
report_lines.append('')
report_lines.append('| Parameter | Value |')
report_lines.append('|-----------|-------|')
report_lines.append(f'| **OS** | {env.get(\"os\", \"unknown\")} |')
report_lines.append(f'| **CPU** | {env.get(\"cpu_model\", \"unknown\")} |')
report_lines.append(f'| **Cores** | {env.get(\"cpu_cores\", \"unknown\")} |')
report_lines.append(f'| **RAM** | {env.get(\"ram_gb\", \"unknown\")} GB |')
report_lines.append(f'| **Rust** | {env.get(\"rust_version\", \"unknown\")} |')
report_lines.append(f'| **LLVM** | {env.get(\"llvm_version\", \"unknown\")} |')
report_lines.append(f'| **Target** | {env.get(\"target_triple\", \"unknown\")} |')
report_lines.append(f'| **Commit** | {env.get(\"commit_short\", \"unknown\")} |')
report_lines.append(f'| **Branch** | {env.get(\"branch\", \"unknown\")} |')
report_lines.append(f'| **LTO** | {env.get(\"lto\", \"unknown\")} |')
report_lines.append(f'| **Profile** | {env.get(\"profile\", \"unknown\")} |')
report_lines.append('')

report_lines.append('## Performance Results')
report_lines.append('')
report_lines.append('| Benchmark | Duration | Throughput |')
report_lines.append('|-----------|----------|------------|')

summary_data = []
try:
    with open(bench_log) as f:
        content = f.read()
        for match in re.finditer(r'(\S+)\s+(\d+\.\d+)\s+ns', content):
            name, ns = match.group(1), float(match.group(2))
            dur = f'{ns/1e9:.2f} s' if ns > 1e9 else f'{ns/1e6:.2f} ms' if ns > 1e6 else f'{ns/1e3:.2f} us' if ns > 1e3 else f'{ns:.0f} ns'
            thr = f'{1e9/ns:.0f} ops/s' if ns > 0 else 'N/A'
            report_lines.append(f'| {name} | {dur} | {thr} |')
            summary_data.append({'name': name, 'duration_ns': ns, 'throughput_ops_sec': 1e9/ns if ns > 0 else 0})
except Exception as e:
    print(f'Warning: Could not parse benchmark log: {e}')

report_lines.extend(['', '## Summary', '', f'- **Total benchmarks**: {len(summary_data)}', ''])

os.makedirs('${RESULTS_DIR}/reports', exist_ok=True)
with open('${RESULTS_DIR}/reports/KCM_BENCHMARK_REPORT.md', 'w') as f:
    f.write('\n'.join(report_lines))

json_summary = {
    'benchmark_version': '2.0.0',
    'environment': env,
    'results': summary_data,
    'total_benchmarks': len(summary_data),
    'generated': datetime.now(timezone.utc).isoformat()
}
with open('${RESULTS_DIR}/reports/KCM_BENCHMARK_SUMMARY.json', 'w') as f:
    json.dump(json_summary, f, indent=2)

with open('${RESULTS_DIR}/reports/KCM_PERFORMANCE_MATRIX.csv', 'w') as f:
    f.write('benchmark,duration_ns,throughput_ops_sec\n')
    for item in summary_data:
        f.write(f'{item[\"name\"]},{item[\"duration_ns\"]},{item[\"throughput_ops_sec\"]:.0f}\n')

print(f'Reports generated: {len(summary_data)} benchmarks')
"

echo ""
echo "=== Reports Generated ==="
ls -la "${RESULTS_DIR}/reports/"
echo ""
echo "=== Done ==="
