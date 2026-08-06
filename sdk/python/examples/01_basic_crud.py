#!/usr/bin/env python3
"""KCM Python SDK — Basic CRUD Example.

Demonstrates: insert, query, update, delete operations on facts.
Each fact has 10 fields: subject, predicate, object, confidence,
evidence, timestamp, context, version, priority, owner.
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, KcmError, ErrorCode


def main() -> None:
    print("=== KCM Python SDK — Basic CRUD Example ===\n")

    db = Database()

    # --- INSERT ---
    print("--- Insert Facts ---")
    row0 = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
    row1 = db.insert(subject=2, predicate=1, object=3, confidence=0.90)
    row2 = db.insert(subject=3, predicate=2, object=4, confidence=0.85,
                     evidence=1, context=1, version=1, priority=0, owner=1)
    row3 = db.insert(subject=1, predicate=3, object=5, confidence=0.80,
                     evidence=3, context=2, version=1, priority=-1, owner=7)
    print(f"  Inserted rows: {row0}, {row1}, {row2}, {row3}")
    print(f"  Total facts: {db.fact_count()}, Active: {db.active_fact_count()}")

    # --- QUERY ALL ---
    print("\n--- Query All Facts ---")
    all_facts = db.query_all()
    for f in all_facts:
        print(f"  row_id={f['row_id']}  s={f['subject']}  p={f['predicate']}  "
              f"o={f['object']}  c={f['confidence']:.2f}")

    # --- QUERY WITH KQL ---
    print("\n--- KQL Query: SELECT * FROM facts WHERE subject = 1 ---")
    result = db.query("SELECT * FROM facts WHERE subject = 1")
    print(f"  Returned {len(result)} facts")
    for f in result:
        print(f"  row_id={f['row_id']}  s={f['subject']}  p={f['predicate']}  o={f['object']}")

    # --- UPDATE ---
    print("\n--- Update Fact ---")
    db.update(row0, subject=10, predicate=0, object=20, confidence=0.99,
              evidence=5, context=3, version=2, priority=2, owner=10)
    updated = db.query(f"SELECT * FROM facts WHERE row_id = {row0}")
    updated_list = updated.collect()
    print(f"  Updated row {row0}: {updated_list[0]}")

    # --- DELETE ---
    print("\n--- Delete Fact ---")
    deleted = db.delete(row3)
    print(f"  Deleted row {row3}: success={deleted}")
    print(f"  Total: {db.fact_count()}, Active: {db.active_fact_count()}")

    # --- VERIFY COUNTS ---
    print("\n--- Verify Counts ---")
    assert db.fact_count() == 4, f"Expected 4 total, got {db.fact_count()}"
    assert db.active_fact_count() == 3, f"Expected 3 active, got {db.active_fact_count()}"
    print("  Counts verified: 4 total, 3 active")

    db.close()
    print("\n=== All operations completed ===")


if __name__ == "__main__":
    main()
