"""KCM Knowledge Columnar Model - Python SDK

Pure in-memory reference implementation aligned with the SSOT API specification.
In production, this would wrap the PyO3 bindings from kcm-interface.
"""

from __future__ import annotations

import enum
import json
import os
import time
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Dict, Iterator, List, Optional, Sequence

__version__ = "0.1.0"


# ---------------------------------------------------------------------------
# Error codes — mirrors KCM_Error from KCM_API_SPEC.md §2.1
# ---------------------------------------------------------------------------

class ErrorCode(enum.IntEnum):
    """KCM error codes matching the C FFI enum."""
    OK = 0
    NOT_FOUND = 1
    OUT_OF_MEMORY = 2
    INVALID_ARGUMENT = 3
    IO = 4
    CORRUPTED = 5
    CONFLICT = 6
    TRANSACTION_ABORTED = 7


# ---------------------------------------------------------------------------
# Exception hierarchy
# ---------------------------------------------------------------------------

class KcmError(Exception):
    """KCM exception carrying an error code and human-readable message."""

    def __init__(self, code: ErrorCode, message: str) -> None:
        self.code = code
        self.message = message
        super().__init__(f"[{code.name}] {message}")

    def __repr__(self) -> str:
        return f"KcmError({self.code!r}, {self.message!r})"


# ---------------------------------------------------------------------------
# Fact dataclass — all 10 fields from KCM_Fact (KCM_API_SPEC.md §2.1)
# ---------------------------------------------------------------------------

@dataclass
class Fact:
    """Represents a knowledge fact with all SSOT-defined fields.

    Field types follow the C FFI spec:
        subject:  uint32
        predicate: uint8
        object:   uint32
        confidence: double
        evidence:  uint8
        timestamp: int64
        context:   uint8
        version:   int32
        priority:  int8
        owner:     uint16
    """

    subject: int
    predicate: int
    object: int
    confidence: float
    evidence: int = 0
    timestamp: int = 0
    context: int = 0
    version: int = 1
    priority: int = 0
    owner: int = 0

    def __post_init__(self) -> None:
        if not (0.0 <= self.confidence <= 1.0):
            raise KcmError(
                ErrorCode.INVALID_ARGUMENT,
                f"confidence must be in [0, 1], got {self.confidence}",
            )
        if self.timestamp == 0:
            self.timestamp = int(time.time() * 1_000_000_000)

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> Fact:
        return cls(**d)


# ---------------------------------------------------------------------------
# QueryResult — iterator over query results
# ---------------------------------------------------------------------------

class QueryResult:
    """Lazily evaluated query result set supporting iteration and collect."""

    def __init__(self, facts: List[Dict[str, Any]]) -> None:
        self._facts = facts
        self._index = 0

    def __iter__(self) -> Iterator[Dict[str, Any]]:
        return iter(self._facts)

    def __next__(self) -> Dict[str, Any]:
        if self._index >= len(self._facts):
            raise StopIteration
        result = self._facts[self._index]
        self._index += 1
        return result

    def collect(self) -> List[Dict[str, Any]]:
        """Return all results as a list of dicts."""
        return list(self._facts)

    def __len__(self) -> int:
        return len(self._facts)

    def __repr__(self) -> str:
        return f"QueryResult(count={len(self._facts)})"


# ---------------------------------------------------------------------------
# KQL query parser (minimal subset for reference implementation)
# ---------------------------------------------------------------------------

def _parse_kql(kql: str, facts: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """Execute a minimal KQL query against in-memory facts.

    Supported syntax:
        SELECT * FROM facts
        SELECT * FROM facts WHERE subject = <int>
        SELECT * FROM facts WHERE subject = <int> AND predicate = <int>
        SELECT * FROM facts WHERE subject = <int> AND object = <int>
        SELECT * FROM facts WHERE predicate = <int>
        SELECT * FROM facts WHERE object = <int>
    """
    tokens = kql.strip().split()
    if not tokens:
        raise KcmError(ErrorCode.INVALID_ARGUMENT, "empty KQL query")

    upper = kql.upper().strip()
    if not upper.startswith("SELECT"):
        raise KcmError(
            ErrorCode.INVALID_ARGUMENT,
            f"KQL must start with SELECT, got: {kql[:20]}",
        )

    where_idx = upper.find(" WHERE ")
    if where_idx == -1:
        if "FROM FACTS" in upper or "FROM facts" in kql:
            return list(facts)
        raise KcmError(
            ErrorCode.INVALID_ARGUMENT,
            f"KQL must reference 'facts' table: {kql}",
        )

    from_part = upper[:where_idx]
    if "FROM FACTS" not in from_part:
        raise KcmError(
            ErrorCode.INVALID_ARGUMENT,
            f"KQL must reference 'facts' table: {kql}",
        )

    where_clause = kql[where_idx + 7:].strip()
    conditions = [c.strip() for c in where_clause.split(" AND ")]

    results = list(facts)
    for cond in conditions:
        cond = cond.strip()
        op_pos = -1
        for op in ("!=", "=", "<=", ">=", "<", ">"):
            op_pos = cond.find(op)
            if op_pos != -1:
                break
        if op_pos == -1:
            raise KcmError(
                ErrorCode.INVALID_ARGUMENT,
                f"invalid KQL condition: {cond}",
            )
        key = cond[:op_pos].strip()
        op = cond[op_pos : op_pos + (2 if cond[op_pos + 1 : op_pos + 2] in ("=", "!") else 1)]
        value_str = cond[op_pos + len(op) :].strip()

        try:
            value = int(value_str)
        except ValueError:
            raise KcmError(
                ErrorCode.INVALID_ARGUMENT,
                f"KQL values must be integers, got: {value_str}",
            )

        if key not in ("subject", "predicate", "object", "evidence", "context",
                        "version", "priority", "owner", "row_id"):
            raise KcmError(
                ErrorCode.INVALID_ARGUMENT,
                f"unknown KQL field: {key}",
            )

        filtered: List[Dict[str, Any]] = []
        for f in results:
            fv = f.get(key)
            if fv is None:
                continue
            match = False
            if op == "=":
                match = fv == value
            elif op == "!=":
                match = fv != value
            elif op == "<":
                match = fv < value
            elif op == ">":
                match = fv > value
            elif op == "<=":
                match = fv <= value
            elif op == ">=":
                match = fv >= value
            if match:
                filtered.append(f)
        results = filtered

    return results


# ---------------------------------------------------------------------------
# Transaction
# ---------------------------------------------------------------------------

class Transaction:
    """Represents a database transaction.

    In this reference implementation, transactions record operations but
    do not provide true ACID isolation. They serve as an API-compatible stub.
    """

    def __init__(self, db: Database, txn_id: int) -> None:
        self._db = db
        self.txn_id = txn_id
        self._committed = False
        self._rolled_back = False
        self._operations: List[str] = []

    def commit(self) -> None:
        """Commit the transaction."""
        if self._committed:
            raise KcmError(ErrorCode.CONFLICT, "transaction already committed")
        if self._rolled_back:
            raise KcmError(ErrorCode.TRANSACTION_ABORTED, "transaction already rolled back")
        self._committed = True

    def rollback(self) -> None:
        """Roll back the transaction."""
        if self._committed:
            raise KcmError(ErrorCode.CONFLICT, "transaction already committed")
        if self._rolled_back:
            raise KcmError(ErrorCode.CONFLICT, "transaction already rolled back")
        self._rolled_back = True

    def __enter__(self) -> Transaction:
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        if not self._committed and not self._rolled_back:
            self.rollback()

    def __repr__(self) -> str:
        status = "committed" if self._committed else ("rolled back" if self._rolled_back else "active")
        return f"Transaction(id={self.txn_id}, status={status})"


# ---------------------------------------------------------------------------
# Database
# ---------------------------------------------------------------------------

class Database:
    """KCM Knowledge Database — in-memory reference implementation.

    Example::

        db = Database()
        row_id = db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        results = db.query("SELECT * FROM facts WHERE subject = 1")
        db.close()
    """

    def __init__(self) -> None:
        self._facts: List[Dict[str, Any]] = []
        self._deleted: set[int] = set()
        self._next_id: int = 0
        self._txn_counter: int = 0
        self._closed: bool = False

    # -- internal helpers ---------------------------------------------------

    def _check_closed(self) -> None:
        if self._closed:
            raise KcmError(ErrorCode.INVALID_ARGUMENT, "database is closed")

    def _get_row(self, row_id: int) -> Optional[Dict[str, Any]]:
        for f in self._facts:
            if f["row_id"] == row_id:
                return f
        return None

    # -- public API (SSOT-aligned) ------------------------------------------

    def insert(
        self,
        subject: int,
        predicate: int,
        object: int,
        confidence: float,
        *,
        evidence: int = 0,
        timestamp: int = 0,
        context: int = 0,
        version: int = 1,
        priority: int = 0,
        owner: int = 0,
    ) -> int:
        """Insert a fact and return its row_id.

        Raises:
            KcmError: with INVALID_ARGUMENT if confidence is out of range.
        """
        self._check_closed()
        fact = Fact(
            subject=subject,
            predicate=predicate,
            object=object,
            confidence=confidence,
            evidence=evidence,
            timestamp=timestamp,
            context=context,
            version=version,
            priority=priority,
            owner=owner,
        )
        row_id = self._next_id
        d = fact.to_dict()
        d["row_id"] = row_id
        self._facts.append(d)
        self._next_id += 1
        return row_id

    def update(
        self,
        row_id: int,
        subject: int,
        predicate: int,
        object: int,
        confidence: float,
        *,
        evidence: int = 0,
        timestamp: int = 0,
        context: int = 0,
        version: int = 1,
        priority: int = 0,
        owner: int = 0,
    ) -> None:
        """Update an existing fact by row_id.

        Raises:
            KcmError: with NOT_FOUND if row_id does not exist,
                      with INVALID_ARGUMENT if confidence is out of range.
        """
        self._check_closed()
        row = self._get_row(row_id)
        if row is None or row_id in self._deleted:
            raise KcmError(ErrorCode.NOT_FOUND, f"row_id {row_id} not found")
        if not (0.0 <= confidence <= 1.0):
            raise KcmError(
                ErrorCode.INVALID_ARGUMENT,
                f"confidence must be in [0, 1], got {confidence}",
            )
        row["subject"] = subject
        row["predicate"] = predicate
        row["object"] = object
        row["confidence"] = confidence
        row["evidence"] = evidence
        if timestamp != 0:
            row["timestamp"] = timestamp
        row["context"] = context
        row["version"] = version
        row["priority"] = priority
        row["owner"] = owner

    def delete(self, row_id: int) -> bool:
        """Delete a fact by row_id. Returns True if deleted, False if not found."""
        self._check_closed()
        if row_id in self._deleted:
            return False
        row = self._get_row(row_id)
        if row is None:
            return False
        self._deleted.add(row_id)
        return True

    def query(self, kql: str) -> QueryResult:
        """Execute a KQL query string and return a QueryResult.

        Supported KQL::

            SELECT * FROM facts
            SELECT * FROM facts WHERE subject = <int>
            SELECT * FROM facts WHERE subject = <int> AND predicate = <int>
        """
        self._check_closed()
        active = [f for f in self._facts if f["row_id"] not in self._deleted]
        results = _parse_kql(kql, active)
        return QueryResult(results)

    def query_all(self) -> List[Dict[str, Any]]:
        """Convenience method — returns all active facts as a list of dicts."""
        self._check_closed()
        return [f for f in self._facts if f["row_id"] not in self._deleted]

    def fact_count(self) -> int:
        """Return total number of facts (including deleted)."""
        self._check_closed()
        return len(self._facts)

    def active_fact_count(self) -> int:
        """Return number of active (non-deleted) facts."""
        self._check_closed()
        return len(self._facts) - len(self._deleted)

    def begin_transaction(self) -> Transaction:
        """Begin a new transaction and return a Transaction handle."""
        self._check_closed()
        self._txn_counter += 1
        return Transaction(self, self._txn_counter)

    def save(self, path: str) -> None:
        """Persist the database to a JSON file.

        Raises:
            KcmError: with IO if the file cannot be written.
        """
        self._check_closed()
        data = {
            "version": 1,
            "next_id": self._next_id,
            "facts": self._facts,
            "deleted": list(self._deleted),
        }
        try:
            Path(path).parent.mkdir(parents=True, exist_ok=True)
            with open(path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2)
        except OSError as e:
            raise KcmError(ErrorCode.IO, f"failed to save database: {e}") from e

    def load(self, path: str) -> None:
        """Load a database from a JSON file.

        Raises:
            KcmError: with IO if the file cannot be read,
                      with CORRUPTED if the file format is invalid.
        """
        self._check_closed()
        try:
            with open(path, "r", encoding="utf-8") as f:
                data = json.load(f)
        except FileNotFoundError as e:
            raise KcmError(ErrorCode.IO, f"file not found: {path}") from e
        except (json.JSONDecodeError, OSError) as e:
            raise KcmError(ErrorCode.CORRUPTED, f"failed to read database: {e}") from e

        if not isinstance(data, dict) or "facts" not in data:
            raise KcmError(ErrorCode.CORRUPTED, "invalid database file format")

        self._facts = data["facts"]
        self._next_id = data.get("next_id", len(self._facts))
        self._deleted = set(data.get("deleted", []))

    @staticmethod
    def verify(path: str) -> None:
        """Verify the integrity of a saved database file.

        Raises:
            KcmError: with IO if the file cannot be read,
                      with CORRUPTED if the file is invalid.
        """
        try:
            with open(path, "r", encoding="utf-8") as f:
                data = json.load(f)
        except FileNotFoundError as e:
            raise KcmError(ErrorCode.IO, f"file not found: {path}") from e
        except (json.JSONDecodeError, OSError) as e:
            raise KcmError(ErrorCode.CORRUPTED, f"failed to read database: {e}") from e

        if not isinstance(data, dict):
            raise KcmError(ErrorCode.CORRUPTED, "database file is not a JSON object")
        if "facts" not in data or not isinstance(data["facts"], list):
            raise KcmError(ErrorCode.CORRUPTED, "database file missing 'facts' array")
        if "next_id" not in data:
            raise KcmError(ErrorCode.CORRUPTED, "database file missing 'next_id'")

        required_fields = {"subject", "predicate", "object", "confidence",
                           "evidence", "timestamp", "context", "version",
                           "priority", "owner"}
        for i, fact in enumerate(data["facts"]):
            if "row_id" not in fact:
                raise KcmError(ErrorCode.CORRUPTED, f"fact at index {i} missing 'row_id'")
            missing = required_fields - set(fact.keys())
            if missing:
                raise KcmError(
                    ErrorCode.CORRUPTED,
                    f"fact at index {i} missing fields: {missing}",
                )
            conf = fact["confidence"]
            if not isinstance(conf, (int, float)) or not (0.0 <= conf <= 1.0):
                raise KcmError(
                    ErrorCode.CORRUPTED,
                    f"fact at index {i} has invalid confidence: {conf}",
                )

    def close(self) -> None:
        """Close the database and release resources."""
        self._facts.clear()
        self._deleted.clear()
        self._closed = True

    def __repr__(self) -> str:
        return (
            f"Database(facts={self.active_fact_count()}, "
            f"total={self.fact_count()}, deleted={len(self._deleted)})"
        )


__all__ = [
    "ErrorCode",
    "KcmError",
    "Fact",
    "QueryResult",
    "Transaction",
    "Database",
]
