#!/usr/bin/env python3
"""KCM Python SDK — basic usage example.

Demonstrates: insert, query, update, delete, transactions, save/load/verify.
"""

import os
import sys
import tempfile

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, KcmError, ErrorCode


def main() -> None:
    print("=== KCM Python SDK Example ===\n")

    db = Database()
    print("1. Created in-memory database")

    # Insert facts
    db.insert(subject=1, predicate=0, object=2, confidence=0.95)
    db.insert(subject=2, predicate=1, object=3, confidence=0.90)
    db.insert(subject=3, predicate=2, object=4, confidence=0.85)
    db.insert(subject=1, predicate=3, object=5, confidence=0.80, evidence=3, owner=7)
    print(f"2. Inserted 4 facts (count={db.fact_count()}, active={db.active_fact_count()})")

    # Query all
    all_facts = db.query_all()
    print(f"3. query_all() returned {len(all_facts)} facts:")
    for f in all_facts:
        print(f"   row_id={f['row_id']}  s={f['subject']}  p={f['predicate']}  "
              f"o={f['object']}  c={f['confidence']:.2f}")

    # KQL query
    result = db.query("SELECT * FROM facts WHERE subject = 1")
    print(f"\n4. KQL query 'SELECT * FROM facts WHERE subject = 1': {len(result)} results")
    for f in result:
        print(f"   row_id={f['row_id']}  s={f['subject']}  p={f['predicate']}  o={f['object']}")

    # Multi-condition query
    result = db.query("SELECT * FROM facts WHERE subject = 1 AND predicate = 0")
    print(f"\n5. KQL query '...AND predicate = 0': {len(result)} results")

    # Update
    row_id = db.insert(subject=99, predicate=9, object=99, confidence=0.5)
    print(f"\n6. Inserted row {row_id} for update demo")
    db.update(row_id, subject=100, predicate=10, object=100, confidence=0.99, version=2)
    updated = db.query(f"SELECT * FROM facts WHERE row_id = {row_id}")
    print(f"   After update: {updated.collect()[0]}")

    # Delete
    db.delete(row_id)
    print(f"\n7. Deleted row {row_id}: count={db.fact_count()}, active={db.active_fact_count()}")

    # Transaction
    print("\n8. Transaction demo:")
    with db.begin_transaction() as txn:
        print(f"   Began {txn}")
        txn.commit()
        print(f"   Committed {txn}")

    with db.begin_transaction() as txn:
        print(f"   Began {txn}")
        txn.rollback()
        print(f"   Rolled back {txn}")

    # Save / Load / Verify
    path = os.path.join(tempfile.gettempdir(), "kcm_example.json")
    db.save(path)
    print(f"\n9. Saved database to {path}")

    Database.verify(path)
    print("10. Verification passed")

    db2 = Database()
    db2.load(path)
    print(f"11. Loaded database: {db2.fact_count()} facts, {db2.active_fact_count()} active")
    os.unlink(path)

    # Error handling
    print("\n12. Error handling demo:")
    try:
        db.insert(subject=1, predicate=0, object=2, confidence=1.5)
    except KcmError as e:
        print(f"    Caught: {e} (code={e.code.name})")

    try:
        db.update(99999, subject=1, predicate=0, object=2, confidence=0.5)
    except KcmError as e:
        print(f"    Caught: {e} (code={e.code.name})")

    db.close()
    db2.close()
    print("\n=== All examples completed ===")


if __name__ == "__main__":
    main()
