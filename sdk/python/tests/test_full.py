"""Comprehensive tests for KCM Python SDK"""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))
from kcm import Database, Fact

class TestDatabaseCreation:
    def test_empty_database(self):
        db = Database()
        assert db.fact_count() == 0
        assert db.active_fact_count() == 0
        db.close()

    def test_close_and_reopen(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert db.fact_count() == 1
        db.close()
        assert db.fact_count() == 0

class TestInsertion:
    def test_single_insert(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert row == 0
        assert db.fact_count() == 1
        db.close()

    def test_multiple_inserts(self):
        db = Database()
        for i in range(100):
            db.insert(subject=i, predicate=i % 10, object=i * 2, confidence=i / 100.0)
        assert db.fact_count() == 100
        assert db.active_fact_count() == 100
        db.close()

    def test_invalid_confidence_negative(self):
        db = Database()
        try:
            db.insert(subject=1, predicate=0, object=2, confidence=-0.1)
            assert False
        except ValueError:
            pass
        db.close()

    def test_invalid_confidence_over_one(self):
        db = Database()
        try:
            db.insert(subject=1, predicate=0, object=2, confidence=1.5)
            assert False
        except ValueError:
            pass
        db.close()

    def test_boundary_confidence_zero(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=0.0)
        assert row == 0
        db.close()

    def test_boundary_confidence_one(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=1.0)
        assert row == 0
        db.close()

class TestQueries:
    def test_query_all(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        facts = db.query_all()
        assert len(facts) == 2
        assert facts[0] == (1, 0, 2, 0.95)
        assert facts[1] == (2, 1, 3, 0.90)
        db.close()

    def test_query_filter_subject(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=0, object=3, confidence=0.90)
        db.insert(subject=1, predicate=1, object=4, confidence=0.85)
        results = db.query_filter(subject=1)
        assert len(results) == 2
        db.close()

    def test_query_filter_predicate(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        db.insert(subject=3, predicate=0, object=4, confidence=0.85)
        results = db.query_filter(predicate=0)
        assert len(results) == 2
        db.close()

    def test_query_filter_combined(self):
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=1, predicate=1, object=3, confidence=0.90)
        db.insert(subject=2, predicate=0, object=4, confidence=0.85)
        results = db.query_filter(subject=1, predicate=0)
        assert len(results) == 1
        db.close()

    def test_query_after_delete(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.delete(row)
        results = db.query_all()
        assert len(results) == 0
        db.close()

class TestDeletion:
    def test_delete_existing(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert db.active_fact_count() == 1
        result = db.delete(row)
        assert result == True
        assert db.active_fact_count() == 0
        assert db.fact_count() == 1
        db.close()

    def test_delete_nonexistent(self):
        db = Database()
        result = db.delete(999)
        assert result == False
        db.close()

    def test_double_delete(self):
        db = Database()
        row = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.delete(row)
        result = db.delete(row)
        assert result == False
        db.close()

class TestDictionary:
    def test_insert_and_lookup(self):
        db = Database()
        id1 = db.dict_insert_subject("planet")
        id2 = db.dict_insert_subject("star")
        assert id1 != id2
        assert db.dict_lookup_subject("planet") == id1
        assert db.dict_lookup_subject("star") == id2
        db.close()

    def test_insert_duplicate(self):
        db = Database()
        id1 = db.dict_insert_subject("planet")
        id2 = db.dict_insert_subject("planet")
        assert id1 == id2
        db.close()

    def test_get_subject(self):
        db = Database()
        db.dict_insert_subject("galaxy")
        result = db.dict_get_subject(0)
        assert result == "galaxy"
        db.close()

    def test_get_nonexistent(self):
        db = Database()
        result = db.dict_get_subject(999)
        assert result is None
        db.close()

class TestFactClass:
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

    def test_inequality(self):
        f1 = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        f2 = Fact(subject=2, predicate=0, object=2, confidence=0.95)
        assert f1 != f2

    def test_repr(self):
        f = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        r = repr(f)
        assert "Fact" in r
        assert "subject=1" in r

    def test_invalid_confidence(self):
        try:
            Fact(subject=1, predicate=0, object=2, confidence=1.5)
            assert False
        except ValueError:
            pass

class TestStress:
    def test_large_batch_insert(self):
        db = Database()
        for i in range(10000):
            db.insert(subject=i % 1000, predicate=i % 10, object=i % 500, confidence=(i % 100) / 100.0)
        assert db.fact_count() == 10000
        assert db.active_fact_count() == 10000
        db.close()

    def test_large_batch_delete(self):
        db = Database()
        rows = []
        for i in range(1000):
            rows.append(db.insert(subject=i, predicate=0, object=i, confidence=0.5))
        for row in rows:
            db.delete(row)
        assert db.fact_count() == 1000
        assert db.active_fact_count() == 0
        db.close()
