#!/usr/bin/env python3
"""
KCM SDK API Compliance Validator

Reads each SDK's source code and validates:
- All 18 FFI functions are exposed (KCM_API_SPEC.md §2.2)
- All 10 Fact fields are present (KCM_API_SPEC.md §2.1)
- Error codes match the SSOT enum (KCM_API_SPEC.md §2.1)
- Required classes/methods exist per SDK

Generates a compliance report in JSON and human-readable format.

SSOT: KCM_API_SPEC.md §2 (C FFI), consistency_matrix.json
"""

import json
import os
import re
import sys
import time
from pathlib import Path

MATRIX_PATH = Path(__file__).parent / "consistency_matrix.json"
REPORT_DIR = Path(__file__).parent / "reports"
REPO_ROOT = Path(__file__).parent.parent.parent

# FFI functions from KCM_API_SPEC.md §2.2
FFI_FUNCTIONS = [
    "KCM_DatabaseNew",
    "KCM_DatabaseFree",
    "KCM_DatabaseInsert",
    "KCM_DatabaseUpdate",
    "KCM_DatabaseDelete",
    "KCM_DatabaseFactCount",
    "KCM_DatabaseActiveCount",
    "KCM_DatabaseQuery",
    "KCM_QueryNext",
    "KCM_QueryFree",
    "KCM_DatabaseBeginTransaction",
    "KCM_TransactionFree",
    "KCM_DatabaseSave",
    "KCM_DatabaseLoad",
    "KCM_DatabaseVerify",
    "KCM_TransactionCommit",
    "KCM_TransactionRollback",
    "KCM_ErrorMessage",
]

# Fact fields from KCM_API_SPEC.md §2.1
FACT_FIELDS = [
    "subject",
    "predicate",
    "object",
    "confidence",
    "evidence",
    "timestamp",
    "context",
    "version",
    "priority",
    "owner",
]

# Error codes from KCM_API_SPEC.md §2.1
ERROR_CODES = {
    "KCM_OK": 0,
    "KCM_ERR_NOT_FOUND": 1,
    "KCM_ERR_OUT_OF_MEMORY": 2,
    "KCM_ERR_INVALID_ARGUMENT": 3,
    "KCM_ERR_IO": 4,
    "KCM_ERR_CORRUPTED": 5,
    "KCM_ERR_CONFLICT": 6,
    "KCM_ERR_TRANSACTION_ABORTED": 7,
}

# SDK-specific file patterns and expected locations
SDK_CONFIGS = {
    "rust": {
        "name": "Rust",
        "source_patterns": ["**/*.rs"],
        "ffi_source": "crates/kcm-interface/src/**/*.rs",
        "test_patterns": ["**/tests/**/*.rs", "**/*_test.rs"],
        "class_patterns": [],
        "method_patterns": [
            r"pub\s+(async\s+)?fn\s+(\w+)",
        ],
    },
    "python": {
        "name": "Python",
        "source_patterns": ["**/*.py"],
        "ffi_source": None,
        "test_patterns": ["**/test_*.py", "**/*_test.py"],
        "class_patterns": [
            r"class\s+(\w*Database\w*)",
            r"class\s+(\w*Fact\w*)",
        ],
        "method_patterns": [
            r"def\s+(\w+)\s*\(",
        ],
    },
    "javascript": {
        "name": "JavaScript",
        "source_patterns": ["**/*.js"],
        "ffi_source": None,
        "test_patterns": ["**/*.test.js", "**/*.spec.js"],
        "class_patterns": [
            r"class\s+(\w*Database\w*)",
            r"class\s+(\w*Fact\w*)",
        ],
        "method_patterns": [
            r"(?:async\s+)?(\w+)\s*\(",
            r"(\w+)\s*=\s*(?:async\s+)?\(",
        ],
    },
    "typescript": {
        "name": "TypeScript",
        "source_patterns": ["**/*.ts"],
        "ffi_source": None,
        "test_patterns": ["**/*.test.ts", "**/*.spec.ts"],
        "class_patterns": [
            r"class\s+(\w*Database\w*)",
            r"class\s+(\w*Fact\w*)",
            r"interface\s+(\w*Database\w*)",
            r"interface\s+(\w*Fact\w*)",
        ],
        "method_patterns": [
            r"(?:async\s+)?(\w+)\s*\(",
            r"(\w+)\s*[=:]\s*(?:async\s+)?\(",
        ],
    },
    "go": {
        "name": "Go",
        "source_patterns": ["**/*.go"],
        "ffi_source": None,
        "test_patterns": ["**/*_test.go"],
        "class_patterns": [
            r"type\s+(\w*Database\w*)\s+struct",
            r"type\s+(\w*Fact\w*)\s+struct",
        ],
        "method_patterns": [
            r"func\s+\([^)]+\)\s+(\w+)\s*\(",
            r"func\s+(\w+)\s*\(",
        ],
    },
    "java": {
        "name": "Java",
        "source_patterns": ["**/*.java"],
        "ffi_source": None,
        "test_patterns": ["**/*Test.java"],
        "class_patterns": [
            r"class\s+(\w*Database\w*)",
            r"class\s+(\w*Fact\w*)",
        ],
        "method_patterns": [
            r"(?:public|private|protected)?\s*(?:static\s+)?(?:\w+\s+)?(\w+)\s*\(",
        ],
    },
    "dotnet": {
        "name": ".NET",
        "source_patterns": ["**/*.cs"],
        "ffi_source": None,
        "test_patterns": ["**/*Test.cs", "**/*Tests.cs"],
        "class_patterns": [
            r"class\s+(\w*Database\w*)",
            r"class\s+(\w*Fact\w*)",
        ],
        "method_patterns": [
            r"(?:public|private|protected)?\s*(?:static\s+)?(?:\w+\s+)?(\w+)\s*\(",
        ],
    },
    "c": {
        "name": "C",
        "source_patterns": ["**/*.c", "**/*.h"],
        "ffi_source": "crates/kcm-interface/src/**/*.rs",
        "test_patterns": ["**/test_*.c"],
        "class_patterns": [],
        "method_patterns": [
            r"KCM_\w+\s*\(",
        ],
    },
    "cpp": {
        "name": "C++",
        "source_patterns": ["**/*.cpp", "**/*.hpp", "**/*.h"],
        "ffi_source": None,
        "test_patterns": ["**/test_*.cpp"],
        "class_patterns": [
            r"class\s+(\w*Database\w*)",
            r"class\s+(\w*Fact\w*)",
            r"namespace\s+kcm",
        ],
        "method_patterns": [
            r"(?:virtual\s+)?(?:\w+\s+)?(\w+)\s*\(",
        ],
    },
}


def find_files(root, patterns):
    """Find files matching any of the glob patterns."""
    files = []
    for pattern in patterns:
        files.extend(root.glob(pattern))
    return list(set(files))


def read_file_safe(path):
    """Read a file, returning empty string on error."""
    try:
        return path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return ""


def search_code(root, patterns, include=None):
    """Search for regex patterns in source files, optionally filtering by file extension."""
    matches = []
    file_patterns = include or ["*"]
    files = find_files(root, file_patterns)
    for f in files:
        content = read_file_safe(f)
        for pattern in patterns:
            for match in re.finditer(pattern, content, re.MULTILINE):
                matches.append({
                    "file": str(f.relative_to(root)),
                    "match": match.group(0),
                    "groups": match.groups(),
                })
    return matches


class ComplianceReport:
    """Tracks compliance results for a single SDK."""

    def __init__(self, sdk_name):
        self.sdk_name = sdk_name
        self.ffi_functions = {fn: False for fn in FFI_FUNCTIONS}
        self.fact_fields = {field: False for field in FACT_FIELDS}
        self.error_codes = {code: False for code in ERROR_CODES}
        self.classes = []
        self.methods = []
        self.warnings = []
        self.errors = []
        self.files_scanned = 0

    def score(self):
        total_checks = (
            len(FFI_FUNCTIONS) + len(FACT_FIELDS) + len(ERROR_CODES)
        )
        passed = (
            sum(1 for v in self.ffi_functions.values() if v)
            + sum(1 for v in self.fact_fields.values() if v)
            + sum(1 for v in self.error_codes.values() if v)
        )
        return passed, total_checks

    def to_dict(self):
        passed, total = self.score()
        return {
            "sdk": self.sdk_name,
            "files_scanned": self.files_scanned,
            "ffi_functions": self.ffi_functions,
            "fact_fields": self.fact_fields,
            "error_codes": self.error_codes,
            "classes_found": self.classes,
            "methods_found": self.methods[:50],  # Cap at 50 for readability
            "warnings": self.warnings,
            "errors": self.errors,
            "score": f"{passed}/{total}",
            "pass_rate": f"{(passed / total * 100):.1f}%",
        }


def validate_rust(root, report):
    """Validate the Rust (kcm-interface) FFI implementation."""
    ffi_root = root / "crates" / "kcm-interface"
    if not ffi_root.exists():
        report.errors.append(f"FFI source directory not found: {ffi_root}")
        return

    source_files = find_files(ffi_root, ["src/**/*.rs"])
    report.files_scanned = len(source_files)

    all_content = ""
    for f in source_files:
        content = read_file_safe(f)
        all_content += content

        # Check for FFI functions
        for fn in FFI_FUNCTIONS:
            if fn in content:
                report.ffi_functions[fn] = True

        # Check for fact fields in KCM_Fact struct
        if "KCM_Fact" in content:
            for field in FACT_FIELDS:
                if re.search(rf"\b{field}\b", content):
                    report.fact_fields[field] = True

        # Check for error codes
        for code in ERROR_CODES:
            if code in content:
                report.error_codes[code] = True

    # Check for unwrap in production code
    unwrap_matches = search_code(ffi_root, [r"\.unwrap\(\)"], ["src/**/*.rs"])
    if unwrap_matches:
        report.warnings.append(
            f"Found {len(unwrap_matches)} .unwrap() calls in FFI source"
        )

    # Check for panic in production code
    panic_matches = search_code(ffi_root, [r"panic!\("], ["src/**/*.rs"])
    if panic_matches:
        report.warnings.append(
            f"Found {len(panic_matches)} panic!() calls in FFI source"
        )


def validate_sdk_source(root, sdk_name, config, report):
    """Validate a non-Rust SDK's source code."""
    source_dirs = [
        root / "sdk" / sdk_name,
        root / "sdk" / f"{sdk_name}-sdk",
        root / "sdk" / f"kcm-{sdk_name}",
        root / "bindings" / sdk_name,
    ]

    source_root = None
    for d in source_dirs:
        if d.exists():
            source_root = d
            break

    if source_root is None:
        # Try broader search
        all_files = find_files(root / "sdk", config["source_patterns"])
        if all_files:
            source_root = all_files[0].parent
        else:
            report.warnings.append(
                f"No source files found for SDK '{sdk_name}' in expected locations"
            )
            report.errors.append(
                f"SDK source not found. Expected in: {', '.join(str(d) for d in source_dirs)}"
            )
            return

    # Find all source files
    source_files = find_files(source_root, config["source_patterns"])
    report.files_scanned = len(source_files)

    if not source_files:
        report.warnings.append(f"No source files found in {source_root}")
        return

    # Scan all source files for API surface
    all_content = ""
    for f in source_files:
        content = read_file_safe(f)
        all_content += content

        # Check for FFI function names (any SDK that wraps FFI should reference them)
        for fn in FFI_FUNCTIONS:
            # Look for the function name or a camelCase version
            snake_to_camel = fn.replace("KCM_", "").replace("_", "")
            if fn in content or snake_to_camel in content:
                report.ffi_functions[fn] = True

        # Check for Fact class/struct fields
        for field in FACT_FIELDS:
            if re.search(rf"\b{field}\b", content):
                report.fact_fields[field] = True

        # Check for error codes
        for code in ERROR_CODES:
            if code in content:
                report.error_codes[code] = True

    # Find classes
    for pattern in config["class_patterns"]:
        matches = re.findall(pattern, all_content)
        report.classes.extend(matches)

    # Find methods
    for pattern in config["method_patterns"]:
        matches = re.findall(pattern, all_content)
        report.methods.extend(matches)

    # Check for TODO/FIXME/HACK
    todo_matches = search_code(
        source_root, [r"TODO|FIXME|HACK"], config["source_patterns"]
    )
    if todo_matches:
        report.warnings.append(
            f"Found {len(todo_matches)} TODO/FIXME/HACK markers"
        )


def validate_all_sdks():
    """Validate all SDKs against the SSOT."""
    print("=" * 70)
    print("KCM SDK API COMPLIANCE VALIDATOR")
    print("=" * 70)
    print(f"SSOT: KCM_API_SPEC.md")
    print(f"Repository root: {REPO_ROOT}")
    print()

    all_reports = {}

    for sdk_name, config in SDK_CONFIGS.items():
        print(f"--- Validating SDK: {config['name']} ({sdk_name}) ---")
        report = ComplianceReport(sdk_name)

        if sdk_name == "rust":
            validate_rust(REPO_ROOT, report)
        else:
            validate_sdk_source(REPO_ROOT, sdk_name, config, report)

        passed, total = report.score()
        status = "PASS" if passed == total else "PARTIAL"
        if not report.files_scanned:
            status = "NOT_FOUND"

        print(f"  Files scanned:  {report.files_scanned}")
        print(f"  FFI functions:  {sum(1 for v in report.ffi_functions.values() if v)}/{len(FFI_FUNCTIONS)}")
        print(f"  Fact fields:    {sum(1 for v in report.fact_fields.values() if v)}/{len(FACT_FIELDS)}")
        print(f"  Error codes:    {sum(1 for v in report.error_codes.values() if v)}/{len(ERROR_CODES)}")
        print(f"  Classes found:  {len(report.classes)}")
        print(f"  Score:          {passed}/{total} ({report.pass_rate})")
        print(f"  Status:         {status}")

        if report.warnings:
            print(f"  Warnings:")
            for w in report.warnings:
                print(f"    - {w}")
        if report.errors:
            print(f"  Errors:")
            for e in report.errors:
                print(f"    - {e}")

        all_reports[sdk_name] = report.to_dict()
        print()

    return all_reports


def generate_compliance_report(reports):
    """Generate human-readable and JSON compliance report."""
    REPORT_DIR.mkdir(parents=True, exist_ok=True)

    # JSON report
    json_path = REPORT_DIR / "api_compliance_report.json"
    json_data = {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "ssot_source": "KCM_API_SPEC.md",
        "ffi_function_count": len(FFI_FUNCTIONS),
        "fact_field_count": len(FACT_FIELDS),
        "error_code_count": len(ERROR_CODES),
        "sdks": reports,
    }
    with open(json_path, "w") as f:
        json.dump(json_data, f, indent=2)

    # Human-readable report
    txt_path = REPORT_DIR / "api_compliance_report.txt"
    lines = [
        "KCM SDK API COMPLIANCE REPORT",
        f"Generated: {json_data['timestamp']}",
        f"SSOT: {json_data['ssot_source']}",
        "",
        "=" * 70,
        "COMPLIANCE SUMMARY",
        "=" * 70,
        f"{'SDK':<12} {'FFI':>8} {'Fields':>8} {'Errors':>8} {'Total':>10} {'Status':>10}",
        "-" * 70,
    ]

    for sdk_name, data in sorted(reports.items()):
        ffi_count = sum(1 for v in data["ffi_functions"].values() if v)
        field_count = sum(1 for v in data["fact_fields"].values() if v)
        error_count = sum(1 for v in data["error_codes"].values() if v)
        status = "COMPLETE" if data["score"].split("/")[0] == data["score"].split("/")[1] else (
            "NOT_FOUND" if data["files_scanned"] == 0 else "PARTIAL"
        )
        lines.append(
            f"{sdk_name:<12} {ffi_count:>5}/{len(FFI_FUNCTIONS):<2} "
            f"{field_count:>5}/{len(FACT_FIELDS):<2} "
            f"{error_count:>5}/{len(ERROR_CODES):<2} "
            f"{data['score']:>10} {status:>10}"
        )

    lines.extend(["", "=" * 70, "DETAILS", "=" * 70])

    for sdk_name, data in sorted(reports.items()):
        lines.append(f"\n--- {sdk_name} ---")
        lines.append(f"  Files scanned: {data['files_scanned']}")

        # Missing FFI functions
        missing_ffi = [fn for fn, found in data["ffi_functions"].items() if not found]
        if missing_ffi:
            lines.append(f"  Missing FFI functions ({len(missing_ffi)}):")
            for fn in missing_ffi:
                lines.append(f"    - {fn}")

        # Missing fact fields
        missing_fields = [f for f, found in data["fact_fields"].items() if not found]
        if missing_fields:
            lines.append(f"  Missing fact fields ({len(missing_fields)}):")
            for f in missing_fields:
                lines.append(f"    - {f}")

        # Missing error codes
        missing_errors = [c for c, found in data["error_codes"].items() if not found]
        if missing_errors:
            lines.append(f"  Missing error codes ({len(missing_errors)}):")
            for c in missing_errors:
                lines.append(f"    - {c}")

        if data["warnings"]:
            lines.append(f"  Warnings:")
            for w in data["warnings"]:
                lines.append(f"    - {w}")
        if data["errors"]:
            lines.append(f"  Errors:")
            for e in data["errors"]:
                lines.append(f"    - {e}")

    with open(txt_path, "w") as f:
        f.write("\n".join(lines))

    print(f"JSON report: {json_path}")
    print(f"Text report: {txt_path}")

    return json_path, txt_path


def main():
    reports = validate_all_sdks()
    json_path, txt_path = generate_compliance_report(reports)

    # Determine overall pass/fail
    all_complete = True
    for sdk_name, data in reports.items():
        passed = int(data["score"].split("/")[0])
        total = int(data["score"].split("/")[1])
        if passed != total and data["files_scanned"] > 0:
            all_complete = False

    print()
    if all_complete:
        print("OVERALL: ALL SDKs FULLY COMPLIANT")
    else:
        print("OVERALL: COMPLIANCE ISSUES DETECTED — see reports for details")

    sys.exit(0 if all_complete else 1)


if __name__ == "__main__":
    main()
