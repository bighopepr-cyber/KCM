"""Comprehensive tests for the KCM Python SDK (SSOT-aligned)."""

import os
import sys
import tempfile

import pytest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))

from kcm import Database, ErrorCode, Fact, KcmError, QueryResult, Transaction


# =========================================================================
# ErrorCode
# =========================================================================

class TestErrorCode:
    def test_all_codes_exist(self) -> None:
        codes = list(ErrorCode)
        assert len(codes) == 8

    def test_ok_is_zero(self) -> None:
        assert ErrorCode.OK == 0

    def test_codes_are_ints(self) -> None:
        for code in ErrorCode:
            assert isinstance(code, int)


# =========================================================================
# KcmError
# =========================================================================

class TestKcmError:
    def test_construction(self) -> None:
        err = KcmError(ErrorCode.NOT_FOUND, "missing row")
        assert err.code == ErrorCode.NOT_FOUND
        assert err.message == "missing row"
        assert "NOT_FOUND" in str(err)

    def test_repr(self) -> None:
        err = KcmError(ErrorCode.IO, "disk failure")
        r = repr(err)
        assert "KcmError" in r
        assert "IO" in r
        assert "disk failure" in r

    def test_is_exception(self) -> None:
        with pytest.raises(KcmError):
            raise KcmError(ErrorCode.CORRUPTED, "bad data")


# =========================================================================
# Fact dataclass
# =========================================================================

class TestFact:
    def test_default_fields(self) -> None:
        f = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        assert f.subject == 1
        assert f.predicate == 0
        assert f.object == 2
        assert f.confidence == 0.95
        assert f.evidence == 0
        assert f.timestamp != 0  # auto-set
        assert f.context == 0
        assert f.version == 1
        assert f.priority == 0
        assert f.owner == 0

    def test_all_ten_fields(self) -> None:
        f = Fact(
            subject=10, predicate=5, object=20, confidence=0.75,
            evidence=3, timestamp=1234567890, context=2,
            version=7, priority=-1, owner=42,
        )
        assert f.subject == 10
        assert f.predicate == 5
        assert f.object == 20
        assert f.confidence == 0.75
        assert f.evidence == 3
        assert f.timestamp == 1234567890
        assert f.context == 2
        assert f.version == 7
        assert f.priority == -1
        assert f.owner == 42

    def test_boundary_confidence_zero(self) -> None:
        f = Fact(subject=0, predicate=0, object=0, confidence=0.0)
        assert f.confidence == 0.0

    def test_boundary_confidence_one(self) -> None:
        f = Fact(subject=0, predicate=0, object=0, confidence=1.0)
        assert f.confidence == 1.0

    def test_invalid_confidence_below_zero(self) -> None:
        with pytest.raises(KcmError) as exc_info:
            Fact(subject=1, predicate=0, object=2, confidence=-0.1)
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_invalid_confidence_above_one(self) -> None:
        with pytest.raises(KcmError) as exc_info:
            Fact(subject=1, predicate=0, object=2, confidence=1.5)
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_to_dict(self) -> None:
        f = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        d = f.to_dict()
        assert isinstance(d, dict)
        assert d["subject"] == 1
        assert d["confidence"] == 0.95
        assert "row_id" not in d

    def test_from_dict(self) -> None:
        d = {
            "subject": 5, "predicate": 3, "object": 7, "confidence": 0.8,
            "evidence": 1, "timestamp": 999, "context": 2, "version": 3,
            "priority": 0, "owner": 10,
        }
        f = Fact.from_dict(d)
        assert f.subject == 5
        assert f.evidence == 1
        assert f.owner == 10

    def test_equality(self) -> None:
        f1 = Fact(subject=1, predicate=0, object=2, confidence=0.95, timestamp=100)
        f2 = Fact(subject=1, predicate=0, object=2, confidence=0.95, timestamp=100)
        assert f1 == f2

    def test_inequality(self) -> None:
        f1 = Fact(subject=1, predicate=0, object=2, confidence=0.95, timestamp=100)
        f2 = Fact(subject=2, predicate=0, object=2, confidence=0.95, timestamp=100)
        assert f1 != f2

    def test_repr(self) -> None:
        f = Fact(subject=1, predicate=0, object=2, confidence=0.95)
        r = repr(f)
        assert "Fact" in r
        assert "subject=1" in r


# =========================================================================
# QueryResult
# =========================================================================

class TestQueryResult:
    def test_iter(self) -> None:
        qr = QueryResult([{"a": 1}, {"a": 2}])
        items = list(qr)
        assert len(items) == 2

    def test_next(self) -> None:
        qr = QueryResult([{"a": 1}])
        item = next(qr)
        assert item == {"a": 1}
        with pytest.raises(StopIteration):
            next(qr)

    def test_collect(self) -> None:
        qr = QueryResult([{"a": 1}, {"b": 2}])
        collected = qr.collect()
        assert collected == [{"a": 1}, {"b": 2}]

    def test_len(self) -> None:
        qr = QueryResult([{"a": 1}, {"b": 2}, {"c": 3}])
        assert len(qr) == 3

    def test_repr(self) -> None:
        qr = QueryResult([{"a": 1}])
        assert "QueryResult" in repr(qr)


# =========================================================================
# Database — creation and close
# =========================================================================

class TestDatabaseCreation:
    def test_empty_database(self) -> None:
        db = Database()
        assert db.fact_count() == 0
        assert db.active_fact_count() == 0

    def test_close_clears_state(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert db.fact_count() == 1
        db.close()
        with pytest.raises(KcmError):
            db.fact_count()

    def test_operations_after_close_raise(self) -> None:
        db = Database()
        db.close()
        with pytest.raises(KcmError) as exc_info:
            db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT
        assert "closed" in exc_info.value.message

    def test_repr(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        r = repr(db)
        assert "Database" in r
        assert "facts=1" in r


# =========================================================================
# Database — insert
# =========================================================================

class TestInsert:
    def test_single_insert(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert row_id == 0
        assert db.fact_count() == 1
        assert db.active_fact_count() == 1

    def test_multiple_inserts(self) -> None:
        db = Database()
        for i in range(100):
            db.insert(subject=i, predicate=i % 10, object=i * 2, confidence=i / 100.0)
        assert db.fact_count() == 100
        assert db.active_fact_count() == 100

    def test_row_ids_are_sequential(self) -> None:
        db = Database()
        id1 = db.insert(subject=1, predicate=0, object=2, confidence=0.9)
        id2 = db.insert(subject=2, predicate=0, object=3, confidence=0.8)
        id3 = db.insert(subject=3, predicate=0, object=4, confidence=0.7)
        assert id1 == 0
        assert id2 == 1
        assert id3 == 2

    def test_insert_with_optional_fields(self) -> None:
        db = Database()
        row_id = db.insert(
            subject=1, predicate=0, object=2, confidence=0.95,
            evidence=5, context=3, priority=-1, owner=10,
        )
        results = db.query_all()
        assert len(results) == 1
        assert results[0]["evidence"] == 5
        assert results[0]["context"] == 3
        assert results[0]["priority"] == -1
        assert results[0]["owner"] == 10

    def test_boundary_confidence_zero(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.0)
        assert row_id == 0

    def test_boundary_confidence_one(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=1.0)
        assert row_id == 0

    def test_invalid_confidence_negative(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.insert(subject=1, predicate=0, object=2, confidence=-0.1)
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_invalid_confidence_over_one(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.insert(subject=1, predicate=0, object=2, confidence=1.5)
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT


# =========================================================================
# Database — update
# =========================================================================

class TestUpdate:
    def test_update_existing(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        db.update(row_id, subject=10, predicate=1, object=20, confidence=0.9)
        results = db.query_all()
        assert len(results) == 1
        assert results[0]["subject"] == 10
        assert results[0]["predicate"] == 1
        assert results[0]["object"] == 20
        assert results[0]["confidence"] == 0.9

    def test_update_not_found(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.update(999, subject=1, predicate=0, object=2, confidence=0.5)
        assert exc_info.value.code == ErrorCode.NOT_FOUND

    def test_update_deleted_row(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        db.delete(row_id)
        with pytest.raises(KcmError) as exc_info:
            db.update(row_id, subject=10, predicate=1, object=20, confidence=0.9)
        assert exc_info.value.code == ErrorCode.NOT_FOUND

    def test_update_invalid_confidence(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        with pytest.raises(KcmError) as exc_info:
            db.update(row_id, subject=1, predicate=0, object=2, confidence=2.0)
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_update_preserves_row_id(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        db.update(row_id, subject=10, predicate=1, object=20, confidence=0.9)
        results = db.query_all()
        assert results[0]["row_id"] == row_id

    def test_update_increments_version(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5, version=1)
        db.update(row_id, subject=1, predicate=0, object=2, confidence=0.5, version=2)
        results = db.query_all()
        assert results[0]["version"] == 2


# =========================================================================
# Database — delete
# =========================================================================

class TestDelete:
    def test_delete_existing(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        assert db.active_fact_count() == 1
        result = db.delete(row_id)
        assert result is True
        assert db.active_fact_count() == 0
        assert db.fact_count() == 1

    def test_delete_nonexistent(self) -> None:
        db = Database()
        result = db.delete(999)
        assert result is False

    def test_double_delete(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.delete(row_id)
        result = db.delete(row_id)
        assert result is False

    def test_delete_all(self) -> None:
        db = Database()
        rows = []
        for i in range(10):
            rows.append(db.insert(subject=i, predicate=0, object=i, confidence=0.5))
        for row in rows:
            db.delete(row)
        assert db.fact_count() == 10
        assert db.active_fact_count() == 0
        assert len(db.query_all()) == 0


# =========================================================================
# Database — query (KQL)
# =========================================================================

class TestQuery:
    def test_query_all(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        result = db.query("SELECT * FROM facts")
        assert len(result) == 2

    def test_query_where_subject(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=0, object=3, confidence=0.90)
        db.insert(subject=1, predicate=1, object=4, confidence=0.85)
        result = db.query("SELECT * FROM facts WHERE subject = 1")
        assert len(result) == 2

    def test_query_where_predicate(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        db.insert(subject=3, predicate=0, object=4, confidence=0.85)
        result = db.query("SELECT * FROM facts WHERE predicate = 0")
        assert len(result) == 2

    def test_query_where_object(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        result = db.query("SELECT * FROM facts WHERE object = 3")
        assert len(result) == 1

    def test_query_where_and(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=1, predicate=1, object=3, confidence=0.90)
        db.insert(subject=2, predicate=0, object=4, confidence=0.85)
        result = db.query("SELECT * FROM facts WHERE subject = 1 AND predicate = 0")
        assert len(result) == 1

    def test_query_after_delete(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.delete(row_id)
        result = db.query("SELECT * FROM facts")
        assert len(result) == 0

    def test_query_empty(self) -> None:
        db = Database()
        result = db.query("SELECT * FROM facts")
        assert len(result) == 0

    def test_query_invalid_no_select(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.query("INSERT INTO facts VALUES (1, 2)")
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_query_invalid_no_from(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.query("SELECT *")
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_query_invalid_empty(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.query("")
        assert exc_info.value.code == ErrorCode.INVALID_ARGUMENT

    def test_query_result_is_iterable(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        result = db.query("SELECT * FROM facts")
        items = list(result)
        assert len(items) == 1
        assert items[0]["subject"] == 1

    def test_query_result_collect(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        result = db.query("SELECT * FROM facts")
        collected = result.collect()
        assert isinstance(collected, list)
        assert len(collected) == 1


# =========================================================================
# Database — query_all convenience
# =========================================================================

class TestQueryAll:
    def test_returns_all_active(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        all_facts = db.query_all()
        assert len(all_facts) == 2
        assert all_facts[0]["subject"] == 1
        assert all_facts[1]["subject"] == 2

    def test_excludes_deleted(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        db.insert(subject=2, predicate=1, object=3, confidence=0.90)
        db.delete(row_id)
        all_facts = db.query_all()
        assert len(all_facts) == 1

    def test_returns_dicts(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        all_facts = db.query_all()
        assert isinstance(all_facts[0], dict)

    def test_dict_has_all_fields(self) -> None:
        db = Database()
        db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        all_facts = db.query_all()
        f = all_facts[0]
        for key in ("row_id", "subject", "predicate", "object", "confidence",
                     "evidence", "timestamp", "context", "version", "priority", "owner"):
            assert key in f, f"missing field: {key}"


# =========================================================================
# Database — counts
# =========================================================================

class TestCounts:
    def test_fact_count_includes_deleted(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        db.delete(row_id)
        assert db.fact_count() == 1

    def test_active_count_excludes_deleted(self) -> None:
        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        db.delete(row_id)
        assert db.active_fact_count() == 0

    def test_counts_after_multiple_ops(self) -> None:
        db = Database()
        r1 = db.insert(subject=1, predicate=0, object=2, confidence=0.5)
        r2 = db.insert(subject=2, predicate=0, object=3, confidence=0.6)
        r3 = db.insert(subject=3, predicate=0, object=4, confidence=0.7)
        db.delete(r1)
        db.delete(r3)
        assert db.fact_count() == 3
        assert db.active_fact_count() == 1


# =========================================================================
# Transaction
# =========================================================================

class TestTransaction:
    def test_begin_transaction(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        assert isinstance(txn, Transaction)
        assert txn.txn_id == 1

    def test_multiple_transactions_increment_id(self) -> None:
        db = Database()
        t1 = db.begin_transaction()
        t2 = db.begin_transaction()
        assert t2.txn_id == t1.txn_id + 1

    def test_commit(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.commit()
        assert txn._committed is True

    def test_double_commit_raises(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.commit()
        with pytest.raises(KcmError) as exc_info:
            txn.commit()
        assert exc_info.value.code == ErrorCode.CONFLICT

    def test_rollback(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.rollback()
        assert txn._rolled_back is True

    def test_double_rollback_raises(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.rollback()
        with pytest.raises(KcmError) as exc_info:
            txn.rollback()
        assert exc_info.value.code == ErrorCode.CONFLICT

    def test_rollback_after_commit_raises(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.commit()
        with pytest.raises(KcmError) as exc_info:
            txn.rollback()
        assert exc_info.value.code == ErrorCode.CONFLICT

    def test_context_manager_rollback_on_exception(self) -> None:
        db = Database()
        with pytest.raises(RuntimeError):
            with db.begin_transaction() as txn:
                raise RuntimeError("boom")
        assert txn._rolled_back is True

    def test_context_manager_no_exception_still_rollback(self) -> None:
        db = Database()
        with db.begin_transaction() as txn:
            pass
        assert txn._rolled_back is True

    def test_context_manager_explicit_commit(self) -> None:
        db = Database()
        with db.begin_transaction() as txn:
            txn.commit()
        assert txn._committed is True

    def test_repr_active(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        assert "active" in repr(txn)

    def test_repr_committed(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.commit()
        assert "committed" in repr(txn)

    def test_repr_rolled_back(self) -> None:
        db = Database()
        txn = db.begin_transaction()
        txn.rollback()
        assert "rolled back" in repr(txn)


# =========================================================================
# Save / Load / Verify
# =========================================================================

class TestSaveLoad:
    def test_save_and_load(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
            path = f.name
        try:
            db = Database()
            db.insert(subject=1, predicate=0, object=2, confidence=0.95)
            db.insert(subject=2, predicate=1, object=3, confidence=0.90)
            db.save(path)

            db2 = Database()
            db2.load(path)
            assert db2.fact_count() == 2
            assert db2.active_fact_count() == 2
            results = db2.query_all()
            assert results[0]["subject"] == 1
            assert results[1]["subject"] == 2
        finally:
            os.unlink(path)

    def test_save_preserves_deleted(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
            path = f.name
        try:
            db = Database()
            row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
            db.insert(subject=2, predicate=1, object=3, confidence=0.90)
            db.delete(row_id)
            db.save(path)

            db2 = Database()
            db2.load(path)
            assert db2.fact_count() == 2
            assert db2.active_fact_count() == 1
        finally:
            os.unlink(path)

    def test_save_to_nonexistent_dir(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            path = os.path.join(tmpdir, "subdir", "db.json")
            db = Database()
            db.insert(subject=1, predicate=0, object=2, confidence=0.5)
            db.save(path)
            assert os.path.exists(path)

    def test_load_nonexistent_file(self) -> None:
        db = Database()
        with pytest.raises(KcmError) as exc_info:
            db.load("/tmp/kcm_nonexistent_test_file.json")
        assert exc_info.value.code == ErrorCode.IO

    def test_load_corrupted_file(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", mode="w", delete=False) as f:
            f.write("not json {{{")
            path = f.name
        try:
            db = Database()
            with pytest.raises(KcmError) as exc_info:
                db.load(path)
            assert exc_info.value.code == ErrorCode.CORRUPTED
        finally:
            os.unlink(path)

    def test_save_after_close_raises(self) -> None:
        db = Database()
        db.close()
        with pytest.raises(KcmError):
            db.save("/tmp/kcm_test.json")


class TestVerify:
    def test_verify_valid_file(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
            path = f.name
        try:
            db = Database()
            db.insert(subject=1, predicate=0, object=2, confidence=0.95)
            db.save(path)
            Database.verify(path)  # should not raise
        finally:
            os.unlink(path)

    def test_verify_nonexistent_file(self) -> None:
        with pytest.raises(KcmError) as exc_info:
            Database.verify("/tmp/kcm_nonexistent_verify_test.json")
        assert exc_info.value.code == ErrorCode.IO

    def test_verify_corrupted_file(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", mode="w", delete=False) as f:
            f.write("{bad json")
            path = f.name
        try:
            with pytest.raises(KcmError) as exc_info:
                Database.verify(path)
            assert exc_info.value.code == ErrorCode.CORRUPTED
        finally:
            os.unlink(path)

    def test_verify_missing_facts(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", mode="w", delete=False) as f:
            f.write('{"version": 1}')
            path = f.name
        try:
            with pytest.raises(KcmError) as exc_info:
                Database.verify(path)
            assert exc_info.value.code == ErrorCode.CORRUPTED
        finally:
            os.unlink(path)

    def test_verify_missing_row_id(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", mode="w", delete=False) as f:
            f.write('{"facts": [{"subject": 1, "predicate": 0, "object": 2, '
                    '"confidence": 0.95, "evidence": 0, "timestamp": 0, '
                    '"context": 0, "version": 1, "priority": 0, "owner": 0}], '
                    '"next_id": 1, "deleted": []}')
            path = f.name
        try:
            with pytest.raises(KcmError) as exc_info:
                Database.verify(path)
            assert exc_info.value.code == ErrorCode.CORRUPTED
            assert "row_id" in exc_info.value.message
        finally:
            os.unlink(path)

    def test_verify_invalid_confidence(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", mode="w", delete=False) as f:
            f.write('{"facts": [{"row_id": 0, "subject": 1, "predicate": 0, '
                    '"object": 2, "confidence": 2.0, "evidence": 0, "timestamp": 0, '
                    '"context": 0, "version": 1, "priority": 0, "owner": 0}], '
                    '"next_id": 1, "deleted": []}')
            path = f.name
        try:
            with pytest.raises(KcmError) as exc_info:
                Database.verify(path)
            assert exc_info.value.code == ErrorCode.CORRUPTED
        finally:
            os.unlink(path)


# =========================================================================
# Stress tests
# =========================================================================

class TestStress:
    def test_large_batch_insert(self) -> None:
        db = Database()
        for i in range(10_000):
            db.insert(subject=i % 1000, predicate=i % 10, object=i % 500, confidence=(i % 100) / 100.0)
        assert db.fact_count() == 10_000
        assert db.active_fact_count() == 10_000

    def test_large_batch_delete(self) -> None:
        db = Database()
        rows = []
        for i in range(1_000):
            rows.append(db.insert(subject=i, predicate=0, object=i, confidence=0.5))
        for row in rows:
            db.delete(row)
        assert db.fact_count() == 1_000
        assert db.active_fact_count() == 0

    def test_large_save_load(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
            path = f.name
        try:
            db = Database()
            for i in range(1_000):
                db.insert(subject=i, predicate=0, object=i, confidence=0.5)
            db.save(path)

            db2 = Database()
            db2.load(path)
            assert db2.fact_count() == 1_000
        finally:
            os.unlink(path)


# =========================================================================
# Type hints validation (import-time check)
# =========================================================================

class TestTypeHints:
    def test_module_exports(self) -> None:
        import kcm
        assert hasattr(kcm, "Database")
        assert hasattr(kcm, "ErrorCode")
        assert hasattr(kcm, "KcmError")
        assert hasattr(kcm, "Fact")
        assert hasattr(kcm, "QueryResult")
        assert hasattr(kcm, "Transaction")

    def test_version(self) -> None:
        import kcm
        assert isinstance(kcm.__version__, str)
        assert kcm.__version__ == "0.1.0"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
