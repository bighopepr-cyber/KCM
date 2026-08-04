# Observability

| Field | Value |
|-------|-------|
| **Document ID** | KCM-ECO-009 |
| **Title** | Observability |
| **Version** | 1.0.0 |
| **Date** | 2026-08-03 |
| **Status** | Authoritative |
| **Authority** | Engineering Orchestrator (P1) |

---

## 1. Three Pillars

| Pillar | Technology | Purpose |
|--------|-----------|---------|
| Metrics | Prometheus | Time-series numerical data |
| Logs | Structured JSON + Fluentd | Event records |
| Traces | OpenTelemetry + Jaeger | Request flow tracking |

## 2. Metrics

### Built-in Metrics (11 AtomicU64 counters)

| Metric | Type | Description |
|--------|------|-------------|
| facts_inserted | Counter | Total facts inserted |
| facts_deleted | Counter | Total facts deleted |
| facts_queried | Counter | Total facts queried |
| queries_executed | Counter | Total queries executed |
| wal_writes | Counter | WAL write operations |
| wal_replays | Counter | WAL replay operations |
| cache_hits | Counter | Cache hit count |
| cache_misses | Counter | Cache miss count |
| bytes_read | Counter | Total bytes read |
| bytes_written | Counter | Total bytes written |
| errors | Counter | Total errors |

### Prometheus Export

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'kcm'
    static_configs:
      - targets: ['kcm:8080']
    metrics_path: '/metrics'
```

## 3. Logging

### Structured JSON Format

```json
{
  "timestamp": "2026-08-03T17:36:01Z",
  "level": "info",
  "module": "kcm_storage::wal",
  "message": "WAL flushed",
  "fields": {
    "bytes_written": 4096,
    "duration_ms": 12
  }
}
```

### Log Levels

| Level | When to Use |
|-------|-------------|
| error | Operation failed |
| warn | Operation degraded |
| info | Normal operation |
| debug | Detailed debugging |
| trace | Maximum verbosity |

## 4. Distributed Tracing

```rust
use tracing::{info_span, Instrument};

let span = info_span!("query_execute", query_id = %query_id);
let _guard = span.enter();
// All operations within this scope are traced
```

## 5. Dashboards

| Dashboard | Purpose |
|-----------|---------|
| Overview | Key metrics at a glance |
| Query Performance | Query latency and throughput |
| Storage | Disk usage and I/O |
| Memory | Heap and allocation tracking |
| WAL | Write-Ahead Log statistics |

## 6. Alerting Rules

| Alert | Condition | Severity |
|-------|-----------|----------|
| HighErrorRate | errors > 100/min | Critical |
| HighLatency | p99 > 100ms | Warning |
| LowCacheHit | ratio < 50% | Warning |
| DiskFull | usage > 90% | Critical |
| WALBacklog | entries > 10000 | Warning |
