#!/usr/bin/env python3
"""KCM Python SDK — Transaction Example.

Demonstrates: begin, commit, and rollback scenarios with transactions.
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, KcmError, ErrorCode


def main() -> None:
    print("=== KCM Python SDK — Transaction Example ===\n")

    db = Database()

    # Insert baseline facts
    db.insert(subject=1, predicate=0, object=2, confidence=0.95)
    db.insert(subject=2, predicate=1, object=3, confidence=0.90)
    print(f"Initial: {db.active_fact_count()} active facts\n")

    # --- COMMITTED TRANSACTION ---
    print("--- Committed Transaction ---")
    with db.begin_transaction() as txn:
        print(f"  Began {txn}")
        db.insert(subject=3, predicate=2, object=4, confidence=0.85)
        print(f"  Inserted fact in transaction")
        txn.commit()
        print(f"  Committed {txn}")
    print(f"  After commit: {db.active_fact_count()} active facts")
    assert db.active_fact_count() == 3

    # --- ROLLED BACK TRANSACTION ---
    print("\n--- Rolled Back Transaction ---")
    with db.begin_transaction() as txn:
        print(f"  Began {txn}")
        db.insert(subject=4, predicate=3, object=5, confidence=0.80)
        print(f"  Inserted fact in transaction")
        txn.rollback()
        print(f"  Rolled back {txn}")
    print(f"  After rollback: {db.active_fact_count()} active facts")
    assert db.active_fact_count() == 3

    # --- AUTO-ROLLBACK ON EXCEPTION ---
    print("\n--- Auto-Rollback on Exception ---")
    count_before = db.active_fact_count()
    try:
        with db.begin_transaction() as txn:
            db.insert(subject=5, predicate=4, object=6, confidence=0.70)
            raise ValueError("simulated error")
    except ValueError:
        pass
    print(f"  After exception: {db.active_fact_count()} active facts")
    print(f"  Transaction auto-rolled back: {db.active_fact_count() == count_before}")

    # --- MULTIPLE OPERATIONS IN TRANSACTION ---
    print("\n--- Multiple Operations in Transaction ---")
    with db.begin_transaction() as txn:
        db.insert(subject=10, predicate=0, object=20, confidence=0.99)
        db.insert(subject=30, predicate=1, object=40, confidence=0.88)
        db.insert(subject=50, predicate=2, object=60, confidence=0.77)
        print(f"  3 pending operations")
        txn.commit()
    print(f"  After commit: {db.active_fact_count()} active facts")
    assert db.active_fact_count() == 6

    db.close()
    print("\n=== All transaction operations completed ===")


if __name__ == "__main__":
    main()
