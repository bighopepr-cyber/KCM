#!/usr/bin/env python3
"""KCM Python SDK — Persistence Example.

Demonstrates: save, load, and verify database persistence.
"""

import os
import sys
import tempfile

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, KcmError, ErrorCode


def main() -> None:
    print("=== KCM Python SDK — Persistence Example ===\n")

    path = os.path.join(tempfile.gettempdir(), "kcm_persistence_example.json")

    # --- SAVE DATABASE ---
    print("--- Save Database ---")
    db = Database()
    db.insert(subject=1, predicate=0, object=2, confidence=0.95,
              evidence=1, context=1, version=1, owner=1)
    db.insert(subject=2, predicate=1, object=3, confidence=0.90,
              evidence=2, context=1, version=1, owner=2)
    db.insert(subject=3, predicate=2, object=4, confidence=0.85,
              evidence=3, context=2, version=1, owner=3)
    db.delete(1)  # delete one fact before saving
    print(f"  Facts before save: {db.fact_count()} total, {db.active_fact_count()} active")
    db.save(path)
    print(f"  Saved to {path}")

    # --- VERIFY FILE ---
    print("\n--- Verify Database File ---")
    Database.verify(path)
    print("  Verification passed")

    # --- LOAD INTO NEW DATABASE ---
    print("\n--- Load Into New Database ---")
    db2 = Database()
    db2.load(path)
    print(f"  Loaded: {db2.fact_count()} total, {db2.active_fact_count()} active")
    assert db2.fact_count() == 3
    assert db2.active_fact_count() == 2

    # --- VERIFY DATA INTEGRITY ---
    print("\n--- Verify Data Integrity ---")
    all_facts = db2.query_all()
    for f in all_facts:
        print(f"  row_id={f['row_id']}  s={f['subject']}  p={f['predicate']}  o={f['object']}")
    assert len(all_facts) == 2

    # --- SAVE-LOAD ROUND TRIP ---
    print("\n--- Save-Load Round Trip ---")
    db2.insert(subject=10, predicate=0, object=20, confidence=0.99)
    db2.save(path)
    db3 = Database()
    db3.load(path)
    print(f"  Round-trip: {db3.fact_count()} total, {db3.active_fact_count()} active")
    assert db3.fact_count() == 4
    assert db3.active_fact_count() == 3

    # --- CLEANUP ---
    os.unlink(path)
    db.close()
    db2.close()
    db3.close()
    print("\n=== All persistence operations completed ===")


if __name__ == "__main__":
    main()
