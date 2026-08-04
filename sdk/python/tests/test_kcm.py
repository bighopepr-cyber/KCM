"""Tests for KCM Python SDK"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from kcm import Database, Fact


class TestDatabase:
    def test_creation(self):
        db = Database()
        assert db.fact_count() == 0
        assert db.active_fact_count() == 0

    def test_insert(self):
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert row_id == 0
        assert db.fact_count() == 1
        assert db.active_fact_count() == 1

    def test_insert_multiple(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        assert db.fact_count() == 2

    def test_query_all(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        facts = db.query_all()
        assert len(facts) == 2
        assert facts[0] == (1, 0, 2, 0.95)
        assert facts[1] == (2, 1, 3, 0.90)

    def test_query_filter(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=0, object=3, confidence=0.90)
        db.insert(subject=1, predicate=1, object=4, confidence=0.85)
        facts = db.query_filter(subject=1)
        assert len(facts) == 2

    def test_delete(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert db.active_fact_count() == 1
        db.delete(row)
        assert db.fact_count() == 1
        assert db.active_fact_count() == 0
        assert len(db.query_all()) == 0

    def test_dict_insert(self):
        db = Database()
        id1 = db.dict_insert_subject("planet")
        id2 = db.dict_insert_subject("star")
        assert id1 == 0
        assert id2 == 1
        assert db.dict_lookup_subject("planet") == 0
        assert db.dict_get_subject(1) == "star"

    def test_invalid_confidence(self):
        db = Database()
        try:
            db.insert(subject=1, predicate=0, object=2, confidence=1.5)
            assert False, "Should have raised ValueError"
        except ValueError:
            pass


class TestFact:
    def test_creation(self):
        f = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        assert f.subject == 1
        assert f.predicate == 0
        assert f.object == 2
        assert f.confidence == 0.95

    def test_equality(self):
        f1 = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        f2 = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        assert f1 == f2

    def test_repr(self):
        f = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        assert "Fact" in repr(f)


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
