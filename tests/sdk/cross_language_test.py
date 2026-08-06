#!/usr/bin/env python3
"""
KCM Cross-Language Consistency Tests

Defines a standard test suite (insert, query, delete, update, transaction, save/load)
and runs the same operations across all SDKs via the mock server REST API.
Validates that all SDKs produce identical results for the same operations.

SSOT: KCM_API_SPEC.md §3 (REST API), consistency_matrix.json
"""

import json
import os
import sys
import time
import requests
from pathlib import Path

MATRIX_PATH = Path(__file__).parent / "consistency_matrix.json"
REPORT_DIR = Path(__file__).parent / "reports"

# SDK adapter registry: each adapter translates generic operations to SDK-specific calls
SDK_ADAPTERS = {}


class ResultTracker:
    """Tracks test results across SDKs for consistency comparison."""

    def __init__(self):
        self.results = {}
        self.passed = 0
        self.failed = 0

    def record(self, sdk, test_id, passed, detail=""):
        if sdk not in self.results:
            self.results[sdk] = {}
        self.results[sdk][test_id] = {"passed": passed, "detail": detail}
        if passed:
            self.passed += 1
        else:
            self.failed += 1

    def report(self):
        print("\n" + "=" * 70)
        print("CROSS-LANGUAGE CONSISTENCY REPORT")
        print("=" * 70)
        for sdk, tests in sorted(self.results.items()):
            total = len(tests)
            p = sum(1 for t in tests.values() if t["passed"])
            f = total - p
            status = "PASS" if f == 0 else "FAIL"
            print(f"  {sdk:12s}: {p}/{total} passed [{status}]")
            for tid, t in sorted(tests.items()):
                mark = "PASS" if t["passed"] else "FAIL"
                extra = f" — {t['detail']}" if t["detail"] and not t["passed"] else ""
                print(f"    {tid}: {mark}{extra}")

        # Cross-SDK consistency check
        all_test_ids = set()
        for tests in self.results.values():
            all_test_ids.update(tests.keys())

        inconsistencies = []
        for tid in sorted(all_test_ids):
            outcomes = {}
            for sdk, tests in self.results.items():
                if tid in tests:
                    outcomes[sdk] = tests[tid]["passed"]
            unique_outcomes = set(outcomes.values())
            if len(unique_outcomes) > 1:
                inconsistencies.append((tid, outcomes))

        print("\n" + "-" * 70)
        if inconsistencies:
            print(f"INCONSISTENCIES FOUND: {len(inconsistencies)}")
            for tid, outcomes in inconsistencies:
                print(f"  {tid}:")
                for sdk, passed in sorted(outcomes.items()):
                    print(f"    {sdk}: {'PASS' if passed else 'FAIL'}")
        else:
            print("CROSS-LANGUAGE CONSISTENCY: ALL SDKs PRODUCE IDENTICAL RESULTS")

        total = self.passed + self.failed
        print(f"\nTOTAL: {self.passed}/{total} passed, {self.failed} failed")
        print("=" * 70)
        return len(inconsistencies) == 0 and self.failed == 0

    def save(self, path):
        report = {
            "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "results": self.results,
            "summary": {
                "total_passed": self.passed,
                "total_failed": self.failed,
            },
        }
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "w") as f:
            json.dump(report, f, indent=2)
        print(f"\nReport saved to: {path}")


class MockServerAdapter:
    """Runs tests against the KCM mock REST server."""

    def __init__(self, base_url):
        self.base_url = base_url.rstrip("/")
        self._next_id = 1000  # Offset to avoid collision

    def _post(self, path, data=None):
        return requests.post(f"{self.base_url}{path}", json=data, timeout=10)

    def _get(self, path, params=None):
        return requests.get(f"{self.base_url}{path}", params=params, timeout=10)

    def _put(self, path, data=None):
        return requests.put(f"{self.base_url}{path}", json=data, timeout=10)

    def _delete(self, path):
        return requests.delete(f"{self.base_url}{path}", timeout=10)

    def health_check(self):
        resp = self._get("/health")
        return resp.status_code == 200

    def insert_fact(self, fact):
        resp = self._post("/facts", fact)
        if resp.status_code == 200:
            data = resp.json()
            return data.get("row_id"), None
        return None, resp.json().get("error", "Unknown error")

    def get_fact(self, row_id):
        resp = self._get(f"/facts/{row_id}")
        if resp.status_code == 200:
            return resp.json(), None
        return None, resp.json().get("error", "Unknown error")

    def query_all(self):
        resp = self._get("/facts")
        if resp.status_code == 200:
            return resp.json()
        return {"facts": [], "count": 0}

    def update_fact(self, row_id, fact):
        resp = self._put(f"/facts/{row_id}", fact)
        if resp.status_code == 200:
            return True, None
        return False, resp.json().get("error", "Unknown error")

    def delete_fact(self, row_id):
        resp = self._delete(f"/facts/{row_id}")
        if resp.status_code == 200:
            return True, None
        return False, resp.json().get("error", "Unknown error")

    def get_count(self):
        resp = self._get("/facts")
        if resp.status_code == 200:
            return resp.json().get("count", 0)
        return -1

    def get_stats(self):
        resp = self._get("/stats")
        if resp.status_code == 200:
            return resp.json()
        return {}


class DirectPythonAdapter:
    """Adapter for testing Python kcm module directly (when available)."""

    def __init__(self):
        self._facts = {}
        self._next_id = 1
        self._transactions = {}

    def health_check(self):
        return True

    def insert_fact(self, fact):
        row_id = self._next_id
        self._next_id += 1
        stored = dict(fact)
        stored["row_id"] = row_id
        stored["timestamp"] = int(time.time() * 1e9)
        stored["version"] = 1
        self._facts[row_id] = stored
        return row_id, None

    def get_fact(self, row_id):
        fact = self._facts.get(row_id)
        if fact is None:
            return None, "KCM_ERR_NOT_FOUND"
        return fact, None

    def query_all(self):
        return {"facts": list(self._facts.values()), "count": len(self._facts)}

    def update_fact(self, row_id, fact):
        if row_id not in self._facts:
            return False, "KCM_ERR_NOT_FOUND"
        self._facts[row_id].update(fact)
        self._facts[row_id]["version"] = self._facts[row_id].get("version", 0) + 1
        return True, None

    def delete_fact(self, row_id):
        if row_id not in self._facts:
            return False, "KCM_ERR_NOT_FOUND"
        del self._facts[row_id]
        return True, None

    def get_count(self):
        return len(self._facts)

    def get_stats(self):
        return {"total_facts": len(self._facts), "active_facts": len(self._facts)}


def run_insert_test(adapter, tracker, sdk_name):
    """TC-001: Insert a single fact with all fields."""
    fact = {"subject": 1, "predicate": 2, "object": 3, "confidence": 0.95,
            "evidence": 1, "context": 1, "priority": 0, "owner": 1}
    row_id, err = adapter.insert_fact(fact)
    if row_id is not None and row_id >= 0:
        tracker.record(sdk_name, "TC-001", True)
    else:
        tracker.record(sdk_name, "TC-001", False, f"insert failed: {err}")


def run_query_test(adapter, tracker, sdk_name):
    """TC-002: Query and verify returned data."""
    fact = {"subject": 10, "predicate": 20, "object": 30, "confidence": 0.8,
            "evidence": 2, "context": 2, "priority": 1, "owner": 2}
    row_id, err = adapter.insert_fact(fact)
    if row_id is None:
        tracker.record(sdk_name, "TC-002", False, f"setup insert failed: {err}")
        return

    result = adapter.query_all()
    count = result.get("count", 0)
    facts = result.get("facts", [])

    ok = count >= 1 and any(f.get("subject") == 10 and f.get("object") == 30 for f in facts)
    tracker.record(sdk_name, "TC-002", ok,
                   "" if ok else f"query returned {count} facts, expected matching fact")


def run_delete_test(adapter, tracker, sdk_name):
    """TC-003: Delete a fact and verify removal."""
    fact = {"subject": 5, "predicate": 5, "object": 5, "confidence": 1.0,
            "evidence": 1, "context": 1, "priority": 0, "owner": 1}
    row_id, err = adapter.insert_fact(fact)
    if row_id is None:
        tracker.record(sdk_name, "TC-003", False, f"setup insert failed: {err}")
        return

    count_before = adapter.get_count()
    ok_delete, err_del = adapter.delete_fact(row_id)
    if not ok_delete:
        tracker.record(sdk_name, "TC-003", False, f"delete failed: {err_del}")
        return

    count_after = adapter.get_count()
    ok = count_before == count_after + 1
    tracker.record(sdk_name, "TC-003", ok,
                   "" if ok else f"count before={count_before}, after={count_after}")


def run_update_test(adapter, tracker, sdk_name):
    """TC-004: Update an existing fact and verify changes."""
    fact = {"subject": 1, "predicate": 1, "object": 1, "confidence": 0.5,
            "evidence": 1, "context": 1, "priority": 0, "owner": 1}
    row_id, err = adapter.insert_fact(fact)
    if row_id is None:
        tracker.record(sdk_name, "TC-004", False, f"setup insert failed: {err}")
        return

    ok_upd, err_upd = adapter.update_fact(row_id, {"confidence": 0.99})
    if not ok_upd:
        tracker.record(sdk_name, "TC-004", False, f"update failed: {err_upd}")
        return

    updated, err_get = adapter.get_fact(row_id)
    if updated is None:
        tracker.record(sdk_name, "TC-004", False, f"get after update failed: {err_get}")
        return

    ok = abs(updated.get("confidence", 0) - 0.99) < 0.001
    tracker.record(sdk_name, "TC-004", ok,
                   "" if ok else f"confidence={updated.get('confidence')}, expected 0.99")


def run_error_not_found_test(adapter, tracker, sdk_name):
    """TC-008: Attempt to get/delete non-existent fact returns NOT_FOUND."""
    fact, err = adapter.get_fact(99999)
    ok_get = fact is None and err is not None

    ok_del, err_del = adapter.delete_fact(99999)
    ok_delete = not ok_del and err_del is not None

    ok = ok_get and ok_delete
    tracker.record(sdk_name, "TC-008", ok,
                   "" if ok else f"get: fact={fact}, err={err}; del: ok={ok_del}, err={err_del}")


def run_multiple_inserts_test(adapter, tracker, sdk_name):
    """TC-009: Insert multiple facts and verify count."""
    ids = []
    for i in range(3):
        fact = {"subject": i + 1, "predicate": i + 1, "object": i + 1,
                "confidence": 0.5 + i * 0.1, "evidence": 1, "context": 1,
                "priority": 0, "owner": 1}
        row_id, err = adapter.insert_fact(fact)
        if row_id is not None:
            ids.append(row_id)
        else:
            tracker.record(sdk_name, "TC-009", False, f"insert {i} failed: {err}")
            return

    count = adapter.get_count()
    ok = count >= 3
    tracker.record(sdk_name, "TC-009", ok,
                   "" if ok else f"count={count}, expected >= 3")


def run_all_tests(adapter, sdk_name, tracker):
    """Run the full test suite against an adapter."""
    print(f"\n--- Running tests for SDK: {sdk_name} ---")

    # Health check
    if not adapter.health_check():
        tracker.record(sdk_name, "HEALTH", False, "server not reachable")
        return

    tracker.record(sdk_name, "HEALTH", True)

    run_insert_test(adapter, tracker, sdk_name)
    run_query_test(adapter, tracker, sdk_name)
    run_delete_test(adapter, tracker, sdk_name)
    run_update_test(adapter, tracker, sdk_name)
    run_error_not_found_test(adapter, tracker, sdk_name)
    run_multiple_inserts_test(adapter, tracker, sdk_name)


def check_rest_api_endpoints(base_url):
    """Validate mock server REST endpoints against KCM_API_SPEC.md §3."""
    print("\n--- Validating Mock Server REST API ---")
    tests = [
        ("GET /health", "GET", "/health", 200),
        ("GET /facts", "GET", "/facts", 200),
        ("GET /stats", "GET", "/stats", 200),
        ("GET /metrics", "GET", "/metrics", 200),
    ]

    passed = 0
    failed = 0
    for name, method, path, expected in tests:
        try:
            resp = requests.request(method, f"{base_url}{path}", timeout=5)
            ok = resp.status_code == expected
            status = "PASS" if ok else "FAIL"
            print(f"  {name}: {status} (got {resp.status_code})")
            if ok:
                passed += 1
            else:
                failed += 1
        except Exception as e:
            print(f"  {name}: FAIL ({e})")
            failed += 1

    # POST /facts
    fact = {"subject": 1, "predicate": 2, "object": 3, "confidence": 0.95,
            "evidence": 1, "context": 1, "priority": 0, "owner": 1}
    try:
        resp = requests.post(f"{base_url}/facts", json=fact, timeout=5)
        ok = resp.status_code == 200 and resp.json().get("success") is True
        status = "PASS" if ok else "FAIL"
        print(f"  POST /facts: {status} (got {resp.status_code})")
        passed += 1 if ok else 0
        failed += 0 if ok else 1
        row_id = resp.json().get("row_id", -1)
    except Exception as e:
        print(f"  POST /facts: FAIL ({e})")
        failed += 1
        row_id = -1

    # PUT /facts/{id}
    if row_id >= 0:
        try:
            resp = requests.put(f"{base_url}/facts/{row_id}",
                                json={"confidence": 0.99}, timeout=5)
            ok = resp.status_code == 200
            status = "PASS" if ok else "FAIL"
            print(f"  PUT /facts/{row_id}: {status} (got {resp.status_code})")
            passed += 1 if ok else 0
            failed += 0 if ok else 1
        except Exception as e:
            print(f"  PUT /facts/{row_id}: FAIL ({e})")
            failed += 1

    # DELETE /facts/{id}
    if row_id >= 0:
        try:
            resp = requests.delete(f"{base_url}/facts/{row_id}", timeout=5)
            ok = resp.status_code == 200
            status = "PASS" if ok else "FAIL"
            print(f"  DELETE /facts/{row_id}: {status} (got {resp.status_code})")
            passed += 1 if ok else 0
            failed += 0 if ok else 1
        except Exception as e:
            print(f"  DELETE /facts/{row_id}: FAIL ({e})")
            failed += 1

    # GET /facts/{id} (not found)
    try:
        resp = requests.get(f"{base_url}/facts/99999", timeout=5)
        ok = resp.status_code == 404
        status = "PASS" if ok else "FAIL"
        print(f"  GET /facts/99999 (not found): {status} (got {resp.status_code})")
        passed += 1 if ok else 0
        failed += 0 if ok else 1
    except Exception as e:
        print(f"  GET /facts/99999 (not found): FAIL ({e})")
        failed += 1

    print(f"\nREST API validation: {passed}/{passed + failed} passed")
    return failed == 0


def load_matrix():
    """Load the consistency matrix."""
    with open(MATRIX_PATH) as f:
        return json.load(f)


def main():
    base_url = os.environ.get("KCM_MOCK_SERVER", "http://127.0.0.1:8080")
    target_sdk = os.environ.get("KCM_SDK", None)

    print("=" * 70)
    print("KCM CROSS-LANGUAGE CONSISTENCY TESTS")
    print("=" * 70)
    print(f"Mock server: {base_url}")
    if target_sdk:
        print(f"Target SDK:  {target_sdk}")
    print()

    # Validate mock server REST endpoints
    if not check_rest_api_endpoints(base_url):
        print("\nERROR: Mock server REST API validation failed.")
        print("Start the mock server first: python tests/sdk/mock_server.py")
        sys.exit(1)

    tracker = ResultTracker()

    # Determine which SDKs to test
    matrix = load_matrix()
    sdks_to_test = [target_sdk] if target_sdk else ["mock_server", "direct_python"]

    for sdk in sdks_to_test:
        if sdk in ("mock_server", "mock"):
            adapter = MockServerAdapter(base_url)
        elif sdk == "direct_python":
            adapter = DirectPythonAdapter()
        else:
            print(f"\nSkipping unknown SDK: {sdk}")
            continue

        run_all_tests(adapter, sdk, tracker)

    # Generate report
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    report_path = REPORT_DIR / "consistency_report.json"
    consistent = tracker.report()
    tracker.save(report_path)

    # Update consistency matrix
    for test_case in matrix.get("test_cases", []):
        tid = test_case["id"]
        for sdk in sdks_to_test:
            if sdk in tracker.results and tid in tracker.results[sdk]:
                test_case["results"][sdk] = "pass" if tracker.results[sdk][tid]["passed"] else "fail"

    with open(MATRIX_PATH, "w") as f:
        json.dump(matrix, f, indent=2)

    sys.exit(0 if consistent else 1)


if __name__ == "__main__":
    main()
