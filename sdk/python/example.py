#!/usr/bin/env python3
"""KCM Python SDK Example"""

import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from kcm import Database, Fact

def main():
    print("=== KCM Python SDK Example ===\n")

    # Create database
    db = Database()
    print("Created database")

    # Insert facts
    db.insert(subject=1, predicate=0, object=2, confidence=0.95)
    db.insert(subject=2, predicate=1, object=3, confidence=0.90)
    db.insert(subject=3, predicate=2, object=4, confidence=0.85)
    print(f"Inserted 3 facts (count={db.fact_count()})")

    # Query all
    facts = db.query_all()
    print(f"\nQuery all ({len(facts)} results):")
    for s, p, o, c in facts:
        print(f"  Subject={s} Predicate={p} Object={o} Confidence={c:.2f}")

    # Query filter
    filtered = db.query_filter(subject=1)
    print(f"\nFiltered by subject=1: {len(filtered)} results")

    # Dictionary
    id1 = db.dict_insert_subject("planet")
    id2 = db.dict_insert_subject("star")
    print(f"\nDictionary: planet={id1}, star={id2}")
    print(f"  Lookup 'planet': {db.dict_lookup_subject('planet')}")
    print(f"  Get id {id2}: {db.dict_get_subject(id2)}")

    # Delete
    row = db.insert(subject=99, predicate=9, object=99, confidence=0.5)
    print(f"\nInserted row {row}, count={db.fact_count()}, active={db.active_fact_count()}")
    db.delete(row)
    print(f"After delete: count={db.fact_count()}, active={db.active_fact_count()}")

    # Close
    db.close()
    print("\nDatabase closed")

    # Stress test
    db2 = Database()
    for i in range(10000):
        db2.insert(subject=i % 1000, predicate=0, object=i, confidence=0.5)
    print(f"Stress test: {db2.fact_count()} facts inserted")
    db2.close()

    print("\nAll Python SDK examples completed!")

if __name__ == "__main__":
    main()
