"""KCM Knowledge Columnar Model - Python SDK

High-level Python wrapper that provides a native-feeling API.
In production, this would wrap the PyO3 bindings from kcm-interface.
"""

from typing import List, Tuple, Optional

__version__ = "0.1.0"


class Database:
    """KCM Knowledge Database.
    
    Example:
        >>> db = Database()
        >>> db.insert(subject=1, predicate=0, object=2, confidence=0.95)
        >>> print(db.fact_count())
        1
    """

    def __init__(self):
        self._facts: list = []
        self._deleted: set = set()
        self._next_id: int = 0
        self._dict_subject: dict = {}
        self._dict_counter: int = 0

    def insert(self, subject: int, predicate: int, object: int, confidence: float) -> int:
        if confidence < 0.0 or confidence > 1.0:
            raise ValueError(f"Confidence must be in [0, 1], got {confidence}")
        row_id = self._next_id
        self._facts.append({
            "subject": subject,
            "predicate": predicate,
            "object": object,
            "confidence": confidence,
            "row_id": row_id,
        })
        self._next_id += 1
        return row_id

    def query_all(self) -> List[Tuple[int, int, int, float]]:
        return [
            (f["subject"], f["predicate"], f["object"], f["confidence"])
            for f in self._facts
            if f["row_id"] not in self._deleted
        ]

    def query_filter(self, subject: Optional[int] = None,
                     predicate: Optional[int] = None,
                     object: Optional[int] = None) -> List[Tuple[int, int, int, float]]:
        results = []
        for f in self._facts:
            if f["row_id"] in self._deleted:
                continue
            if subject is not None and f["subject"] != subject:
                continue
            if predicate is not None and f["predicate"] != predicate:
                continue
            if object is not None and f["object"] != object:
                continue
            results.append((f["subject"], f["predicate"], f["object"], f["confidence"]))
        return results

    def delete(self, row_id: int) -> bool:
        for f in self._facts:
            if f["row_id"] == row_id:
                self._deleted.add(row_id)
                return True
        return False

    def fact_count(self) -> int:
        return len(self._facts)

    def active_fact_count(self) -> int:
        return len(self._facts) - len(self._deleted)

    def dict_insert_subject(self, name: str) -> int:
        if name not in self._dict_subject:
            self._dict_subject[name] = self._dict_counter
            self._dict_counter += 1
        return self._dict_subject[name]

    def dict_lookup_subject(self, name: str) -> Optional[int]:
        return self._dict_subject.get(name)

    def dict_get_subject(self, dict_id: int) -> Optional[str]:
        for name, did in self._dict_subject.items():
            if did == dict_id:
                return name
        return None

    def close(self):
        self._facts.clear()
        self._deleted.clear()


class Fact:
    """Represents a knowledge fact."""

    def __init__(self, subject: int, predicate: int, object: int, confidence: float):
        if confidence < 0.0 or confidence > 1.0:
            raise ValueError(f"Confidence must be in [0, 1], got {confidence}")
        self.subject = subject
        self.predicate = predicate
        self.object = object
        self.confidence = confidence

    def __repr__(self):
        return f"Fact(subject={self.subject}, predicate={self.predicate}, object={self.object}, confidence={self.confidence})"

    def __eq__(self, other):
        if not isinstance(other, Fact):
            return False
        return (self.subject == other.subject and
                self.predicate == other.predicate and
                self.object == other.object and
                abs(self.confidence - other.confidence) < 1e-10)
