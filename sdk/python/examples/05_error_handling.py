#!/usr/bin/env python3
"""KCM Python SDK — Error Handling Example.

Demonstrates: proper error handling patterns with KcmError and ErrorCode.
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, KcmError, ErrorCode


def main() -> None:
    print("=== KCM Python SDK — Error Handling Example ===\n")

    db = Database()

    # --- INVALID CONFIDENCE ---
    print("--- Invalid Confidence (out of range) ---")
    try:
        db.insert(subject=1, predicate=0, object=2, confidence=1.5)
        print("  FAIL: Should have raised")
    except KcmError as e:
        print(f"  Caught: {e}")
        print(f"  Code: {e.code.name} ({e.code.value})")
        assert e.code == ErrorCode.INVALID_ARGUMENT

    # --- NOT FOUND (update non-existent row) ---
    print("\n--- Not Found (update non-existent row) ---")
    try:
        db.update(99999, subject=1, predicate=0, object=2, confidence=0.5)
        print("  FAIL: Should have raised")
    except KcmError as e:
        print(f"  Caught: {e}")
        print(f"  Code: {e.code.name} ({e.code.value})")
        assert e.code == ErrorCode.NOT_FOUND

    # --- NOT FOUND (delete non-existent row) ---
    print("\n--- Not Found (delete non-existent row) ---")
    result = db.delete(99999)
    print(f"  Delete returned: {result} (not an exception for delete)")

    # --- INVALID KQL QUERY ---
    print("\n--- Invalid KQL Query ---")
    try:
        db.query("INVALID QUERY")
        print("  FAIL: Should have raised")
    except KcmError as e:
        print(f"  Caught: {e}")
        print(f"  Code: {e.code.name} ({e.code.value})")

    # --- EMPTY KQL QUERY ---
    print("\n--- Empty KQL Query ---")
    try:
        db.query("")
        print("  FAIL: Should have raised")
    except KcmError as e:
        print(f"  Caught: {e}")
        print(f"  Code: {e.code.name} ({e.code.value})")

    # --- DATABASE CLOSED ---
    print("\n--- Database Closed ---")
    db2 = Database()
    db2.close()
    try:
        db2.insert(subject=1, predicate=0, object=2, confidence=0.5)
        print("  FAIL: Should have raised")
    except KcmError as e:
        print(f"  Caught: {e}")
        print(f"  Code: {e.code.name} ({e.code.value})")

    # --- FILE NOT FOUND (load) ---
    print("\n--- File Not Found (load) ---")
    try:
        db.load("/nonexistent/path/db.json")
        print("  FAIL: Should have raised")
    except KcmError as e:
        print(f"  Caught: {e}")
        print(f"  Code: {e.code.name} ({e.code.value})")

    # --- VERIFY ALL ERROR CODES ---
    print("\n--- All Error Codes ---")
    for code in ErrorCode:
        print(f"  {code.name} ({code.value})")

    # --- TRY-EXCEPT PATTERN ---
    print("\n--- Try-Except Pattern ---")
    try:
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        results = db.query("SELECT * FROM facts WHERE subject = 1")
        print(f"  Query returned {len(results)} results")
    except KcmError as e:
        print(f"  Database error: {e.code.name}: {e}")
    except Exception as e:
        print(f"  Unexpected error: {type(e).__name__}: {e}")

    db.close()
    print("\n=== All error handling patterns completed ===")


if __name__ == "__main__":
    main()
