#!/usr/bin/env python3
"""
KCM Mock REST Server

Implements the REST API from KCM_API_SPEC.md §3 with in-memory storage.
Used for SDK integration testing without requiring the real KCM engine.

Endpoints:
  GET  /health         - Health check
  POST /facts          - Insert fact
  GET  /facts          - List/query facts
  GET  /facts/{id}     - Get fact by ID
  PUT  /facts/{id}     - Update fact
  DELETE /facts/{id}   - Delete fact
  GET  /stats          - Metrics (JSON)
  GET  /metrics        - Metrics (Prometheus)

SSOT: KCM_API_SPEC.md §3
"""

import argparse
import json
import time
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import re

# Error codes matching KCM_API_SPEC.md §2.1
KCM_ERRORS = {
    0: "KCM_OK",
    1: "KCM_ERR_NOT_FOUND",
    2: "KCM_ERR_OUT_OF_MEMORY",
    3: "KCM_ERR_INVALID_ARGUMENT",
    4: "KCM_ERR_IO",
    5: "KCM_ERR_CORRUPTED",
    6: "KCM_ERR_CONFLICT",
    7: "KCM_ERR_TRANSACTION_ABORTED",
}

HTTP_STATUS_MAP = {
    1: 404,   # NOT_FOUND
    2: 507,   # OUT_OF_MEMORY
    3: 400,   # INVALID_ARGUMENT
    4: 500,   # IO
    5: 500,   # CORRUPTED
    6: 409,   # CONFLICT
    7: 409,   # TRANSACTION_ABORTED
}

# Required Fact fields from KCM_API_SPEC.md §2.1
FACT_FIELDS = [
    "subject", "predicate", "object", "confidence",
    "evidence", "context", "priority", "owner"
]


class MockDatabase:
    """In-memory storage implementing KCM database semantics."""

    def __init__(self):
        self._facts = {}
        self._next_id = 1
        self._lock = threading.Lock()
        self._metrics = {
            "queries_total": 0,
            "queries_failed": 0,
            "avg_query_latency_ms": 0.0,
            "inserts_total": 0,
            "inserts_failed": 0,
            "cache_hit_ratio": 0.0,
            "memory_bytes": 0,
            "inferences_total": 0,
            "facts_inferred": 0,
            "total_facts": 0,
            "active_facts": 0,
            "tombstone_count": 0,
        }

    def insert(self, fact):
        with self._lock:
            row_id = self._next_id
            self._next_id += 1
            stored = {
                "row_id": row_id,
                "subject": fact.get("subject", 0),
                "predicate": fact.get("predicate", 0),
                "object": fact.get("object", 0),
                "confidence": fact.get("confidence", 1.0),
                "evidence": fact.get("evidence", 0),
                "timestamp": int(time.time() * 1e9),
                "context": fact.get("context", 0),
                "version": 1,
                "priority": fact.get("priority", 0),
                "owner": fact.get("owner", 0),
                "deleted": False,
            }
            self._facts[row_id] = stored
            self._metrics["inserts_total"] += 1
            self._update_counts()
            return row_id

    def get(self, row_id):
        with self._lock:
            fact = self._facts.get(row_id)
            if fact is None or fact["deleted"]:
                return None, 1  # NOT_FOUND
            return fact, 0

    def update(self, row_id, fact):
        with self._lock:
            existing = self._facts.get(row_id)
            if existing is None or existing["deleted"]:
                return 1  # NOT_FOUND
            for key in FACT_FIELDS:
                if key in fact:
                    existing[key] = fact[key]
            existing["version"] = existing.get("version", 0) + 1
            existing["timestamp"] = int(time.time() * 1e9)
            self._facts[row_id] = existing
            return 0

    def delete(self, row_id):
        with self._lock:
            existing = self._facts.get(row_id)
            if existing is None or existing["deleted"]:
                return 1  # NOT_FOUND
            existing["deleted"] = True
            self._metrics["tombstone_count"] += 1
            self._update_counts()
            return 0

    def list_all(self):
        with self._lock:
            self._metrics["queries_total"] += 1
            return [f for f in self._facts.values() if not f["deleted"]]

    def query(self, subject=None, predicate=None, obj=None, min_confidence=None):
        with self._lock:
            self._metrics["queries_total"] += 1
            results = []
            for fact in self._facts.values():
                if fact["deleted"]:
                    continue
                if subject is not None and fact["subject"] != subject:
                    continue
                if predicate is not None and fact["predicate"] != predicate:
                    continue
                if obj is not None and fact["object"] != obj:
                    continue
                if min_confidence is not None and fact["confidence"] < min_confidence:
                    continue
                results.append(fact)
            return results

    def fact_count(self):
        with self._lock:
            return sum(1 for f in self._facts.values() if not f["deleted"])

    def active_count(self):
        with self._lock:
            return sum(1 for f in self._facts.values() if not f["deleted"])

    def get_stats(self):
        with self._lock:
            self._update_counts()
            return dict(self._metrics)

    def _update_counts(self):
        active = sum(1 for f in self._facts.values() if not f["deleted"])
        self._metrics["total_facts"] = len(self._facts)
        self._metrics["active_facts"] = active
        self._metrics["memory_bytes"] = len(self._facts) * 256


class KCMRequestHandler(BaseHTTPRequestHandler):
    """HTTP request handler implementing KCM REST API."""

    db = MockDatabase()

    def log_message(self, format, *args):
        pass  # Suppress request logging in test mode

    def _send_json(self, status, data):
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())

    def _send_error(self, status, message):
        self._send_json(status, {"error": message})

    def _read_body(self):
        length = int(self.headers.get("Content-Length", 0))
        if length == 0:
            return {}
        return json.loads(self.rfile.read(length))

    def do_GET(self):
        parsed = urlparse(self.path)
        path = parsed.path
        params = parse_qs(parsed.query)

        if path == "/health":
            self._send_json(200, {"status": "healthy"})

        elif path == "/facts":
            # Query with optional filters
            subject = int(params["subject"][0]) if "subject" in params else None
            predicate = int(params["predicate"][0]) if "predicate" in params else None
            obj = int(params["object"][0]) if "object" in params else None
            min_conf = float(params["min_confidence"][0]) if "min_confidence" in params else None

            if any(v is not None for v in [subject, predicate, obj, min_conf]):
                facts = self.db.query(subject, predicate, obj, min_conf)
            else:
                facts = self.db.list_all()

            self._send_json(200, {"facts": facts, "count": len(facts)})

        elif re.match(r"^/facts/(\d+)$", path):
            row_id = int(path.split("/")[-1])
            fact, err = self.db.get(row_id)
            if err != 0:
                self._send_error(HTTP_STATUS_MAP[err], KCM_ERRORS[err])
            else:
                self._send_json(200, fact)

        elif path == "/stats":
            stats = self.db.get_stats()
            stats["estimated_memory_bytes"] = stats["memory_bytes"] * 2
            self._send_json(200, stats)

        elif path == "/metrics":
            stats = self.db.get_stats()
            lines = []
            for key, val in stats.items():
                metric_name = f"kcm_{key}"
                if isinstance(val, float):
                    lines.append(f"# TYPE {metric_name} gauge")
                    lines.append(f"{metric_name} {val}")
                else:
                    lines.append(f"# TYPE {metric_name} gauge")
                    lines.append(f"{metric_name} {val}")
            self.send_response(200)
            self.send_header("Content-Type", "text/plain; version=0.0.4")
            self.end_headers()
            self.wfile.write("\n".join(lines).encode())

        else:
            self._send_error(404, "Not Found")

    def do_POST(self):
        parsed = urlparse(self.path)
        if parsed.path != "/facts":
            self._send_error(404, "Not Found")
            return

        body = self._read_body()
        if not body:
            self._send_error(400, KCM_ERRORS[3])  # INVALID_ARGUMENT
            return

        # Validate required fields
        missing = [f for f in FACT_FIELDS if f not in body]
        if missing:
            self._send_error(400, f"Missing fields: {', '.join(missing)}")
            return

        try:
            row_id = self.db.insert(body)
            self._send_json(200, {"row_id": row_id, "success": True})
        except Exception as e:
            self._send_error(500, str(e))

    def do_PUT(self):
        parsed = urlparse(self.path)
        match = re.match(r"^/facts/(\d+)$", parsed.path)
        if not match:
            self._send_error(404, "Not Found")
            return

        row_id = int(match.group(1))
        body = self._read_body()
        err = self.db.update(row_id, body)
        if err != 0:
            self._send_error(HTTP_STATUS_MAP[err], KCM_ERRORS[err])
        else:
            fact, _ = self.db.get(row_id)
            self._send_json(200, {"success": True, "fact": fact})

    def do_DELETE(self):
        parsed = urlparse(self.path)
        match = re.match(r"^/facts/(\d+)$", parsed.path)
        if not match:
            self._send_error(404, "Not Found")
            return

        row_id = int(match.group(1))
        err = self.db.delete(row_id)
        if err != 0:
            self._send_error(HTTP_STATUS_MAP[err], KCM_ERRORS[err])
        else:
            self._send_json(200, {"success": True})


def main():
    parser = argparse.ArgumentParser(description="KCM Mock REST Server")
    parser.add_argument("--port", type=int, default=8080, help="Port to listen on")
    parser.add_argument("--host", default="127.0.0.1", help="Host to bind to")
    args = parser.parse_args()

    server = HTTPServer((args.host, args.port), KCMRequestHandler)
    print(f"KCM Mock Server listening on http://{args.host}:{args.port}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down.")
        server.shutdown()


if __name__ == "__main__":
    main()
