#!/usr/bin/env python3
"""KCM Python SDK — Query Patterns Example.

Demonstrates: different KQL query patterns and filtering options.
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, KcmError, ErrorCode


def main() -> None:
    print("=== KCM Python SDK — Query Patterns Example ===\n")

    db = Database()

    # Insert test data
    db.insert(subject=1, predicate=0, object=2, confidence=0.95, evidence=1, context=1)
    db.insert(subject=2, predicate=1, object=3, confidence=0.90, evidence=2, context=1)
    db.insert(subject=3, predicate=2, object=4, confidence=0.85, evidence=3, context=2)
    db.insert(subject=1, predicate=3, object=5, confidence=0.80, evidence=1, context=2)
    db.insert(subject=4, predicate=0, object=6, confidence=0.75, evidence=2, context=1)
    print(f"Inserted 5 facts\n")

    # --- SELECT ALL ---
    print("--- SELECT * FROM facts ---")
    result = db.query("SELECT * FROM facts")
    print(f"  Returned {len(result)} facts")

    # --- FILTER BY SUBJECT ---
    print("\n--- SELECT * FROM facts WHERE subject = 1 ---")
    result = db.query("SELECT * FROM facts WHERE subject = 1")
    print(f"  Returned {len(result)} facts")
    for f in result:
        print(f"  s={f['subject']}  p={f['predicate']}  o={f['object']}")
    assert len(result) == 2

    # --- FILTER BY PREDICATE ---
    print("\n--- SELECT * FROM facts WHERE predicate = 0 ---")
    result = db.query("SELECT * FROM facts WHERE predicate = 0")
    print(f"  Returned {len(result)} facts")
    for f in result:
        print(f"  s={f['subject']}  p={f['predicate']}  o={f['object']}")
    assert len(result) == 2

    # --- FILTER BY OBJECT ---
    print("\n--- SELECT * FROM facts WHERE object = 4 ---")
    result = db.query("SELECT * FROM facts WHERE object = 4")
    print(f"  Returned {len(result)} facts")
    assert len(result) == 1

    # --- MULTI-CONDITION FILTER ---
    print("\n--- SELECT * FROM facts WHERE subject = 1 AND predicate = 3 ---")
    result = db.query("SELECT * FROM facts WHERE subject = 1 AND predicate = 3")
    print(f"  Returned {len(result)} facts")
    assert len(result) == 1

    # --- QUERY ALL CONVENIENCE ---
    print("\n--- query_all() convenience method ---")
    all_facts = db.query_all()
    print(f"  Returned {len(all_facts)} facts")
    assert len(all_facts) == 5

    # --- ITERATOR PATTERN ---
    print("\n--- Iterator Pattern ---")
    result = db.query("SELECT * FROM facts WHERE subject = 1")
    for fact in result:
        print(f"  s={fact['subject']}  p={fact['predicate']}  o={fact['object']}  c={fact['confidence']:.2f}")

    db.close()
    print("\n=== All query patterns completed ===")


if __name__ == "__main__":
    main()
