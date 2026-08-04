# Operations Guide

Day-to-day operations for KCM administrators.

## Starting KCM

```bash
# Development
RUST_LOG=debug kcm-server --db /path/to/db.kcm

# Production
RUST_LOG=info kcm-server --db /data/kcm.db --bind 0.0.0.0:8080
```

## Health Checks

```bash
# Health endpoint
curl http://localhost:8080/health

# Detailed stats
curl http://localhost:8080/api/stats
```

## Monitoring

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| facts_inserted | Total inserts | - |
| facts_queried | Total queries | - |
| queries_executed | Query count | - |
| errors | Error count | > 100/min |
| wal_writes | WAL operations | - |
| cache_hits | Cache hits | - |
| cache_misses | Cache misses | - |
| bytes_read | Bytes read | - |
| bytes_written | Bytes written | - |

## Log Levels

| Level | When | Example |
|-------|------|---------|
| error | Operation failed | Disk full |
| warn | Degraded performance | High latency |
| info | Normal operations | Fact inserted |
| debug | Detailed tracing | Query plan |
| trace | Maximum verbosity | Every function call |

## Common Tasks

### Check Database Size

```bash
ls -lh /path/to/db.kcm
```

### Verify Integrity

```bash
kcm-doctor check /path/to/db.kcm
```

### Create Backup

```bash
kcm-backup create /path/to/db.kcm
```
