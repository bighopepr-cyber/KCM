# KCM Operational Runbook

## Overview

This runbook provides operational procedures for managing KCM in production environments.

## Table of Contents

1. [Service Lifecycle](#service-lifecycle)
2. [Health Monitoring](#health-monitoring)
3. [Backup and Recovery](#backup-and-recovery)
4. [Performance Tuning](#performance-tuning)
5. [Troubleshooting](#troubleshooting)
6. [Security Operations](#security-operations)
7. [Scaling Procedures](#scaling-procedures)

---

## Service Lifecycle

### Starting KCM

```bash
# Single node
kcm-server --bind 0.0.0.0:8080 --data-path /data/kcm.db

# Docker
docker run -d -p 8080:8080 -v kcm_data:/data kcm/kcm-server:latest

# Kubernetes
kubectl apply -f deployment/k8s/deployment.yaml
```

### Stopping KCM

```bash
# Graceful shutdown (drains WAL, syncs data)
kill -SIGTERM <PID>

# Force stop (data integrity maintained via WAL)
kill -SIGKILL <PID>
```

### Health Check Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Returns health status (healthy/degraded/unhealthy) |
| `/metrics` | GET | Prometheus metrics endpoint |
| `/ready` | GET | Readiness probe (checks data store availability) |

---

## Health Monitoring

### Key Metrics

| Metric | Warning Threshold | Critical Threshold |
|--------|-------------------|-------------------|
| `kcm_queries_failed_total` | > 10/min | > 50/min |
| `kcm_query_avg_latency_ms` | > 100ms | > 500ms |
| `kcm_cache_hit_ratio` | < 0.5 | < 0.2 |
| `kcm_memory_bytes` | > 1GB | > 2GB |

### Prometheus Alerts

- **KCMHighErrorRate**: Error rate exceeds 10 errors/sec for 5 minutes
- **KCMHighLatency**: Average query latency exceeds 100ms for 5 minutes
- **KCMLowCacheHitRatio**: Cache hit ratio below 0.5 for 10 minutes
- **KCMHighMemory**: Memory usage exceeds 1GB for 5 minutes

---

## Backup and Recovery

### Creating Backups

```bash
# Using CLI
kcm-backup create --path /data/kcm.db --output /backups/kcm-$(date +%Y%m%d).bak

# Using API
curl -X POST http://localhost:8080/api/v1/backup
```

### Restoring from Backup

```bash
# Stop KCM first
kcm-backup restore --backup /backups/kcm-20260101.bak --path /data/kcm.db
# Restart KCM
```

### WAL Replay

KCM uses Write-Ahead Logging (WAL) for crash recovery. On startup, incomplete transactions are automatically replayed.

---

## Performance Tuning

### Memory Configuration

| Parameter | Default | Recommended |
|-----------|---------|-------------|
| `KCM_CACHE_SIZE` | 256MB | 512MB-2GB |
| `KCM_WAL_BUFFER` | 64KB | 256KB |
| `KCM_COMPRESSION_LEVEL` | 3 | 1-5 (trade-off: speed vs compression) |

### Query Optimization

- Use column pruning to reduce I/O
- Enable filter pushdown for predicate-heavy queries
- Leverage zone maps for range queries
- Use bloom filters for equality checks

---

## Troubleshooting

### Common Issues

| Symptom | Cause | Resolution |
|---------|-------|------------|
| High latency | Large dataset, missing indexes | Add bitmap index, increase cache |
| WAL growing large | Long-running transactions | Commit/rollback pending transactions |
| Memory pressure | Large query results | Reduce query limit, enable streaming |
| Connection refused | Port binding, firewall | Check bind address, security groups |

### Log Analysis

```bash
# Enable debug logging
RUST_LOG=debug kcm-server

# Filter WAL operations
RUST_LOG=kcm_storage::wal=debug kcm-server
```

---

## Security Operations

### Key Management

```bash
# Generate encryption key
kcm-cli security generate-key --name production-key

# Rotate key
kcm-cli security rotate-key --name production-key

# List keys
kcm-cli security list-keys
```

### Audit Log Review

```bash
# View recent audit events
kcm-cli audit log --since 1h --type permission_denied

# Verify audit integrity
kcm-cli audit verify
```

---

## Scaling Procedures

### Horizontal Scaling

1. Deploy additional KCM nodes
2. Configure consistent hash sharding
3. Update shard map via `kcm-cluster shard-update`
4. Verify data distribution with `kcm-cluster status`

### Vertical Scaling

1. Update resource limits in deployment config
2. Restart pods with new resource specifications
3. Monitor memory and CPU utilization

---

## Emergency Procedures

### Data Corruption

1. Stop all KCM instances immediately
2. Verify WAL integrity: `kcm-doctor check --wal-verify`
3. Restore from last known good backup
4. Replay WAL from backup point
5. Validate data integrity

### Security Breach

1. Rotate all encryption keys immediately
2. Review audit logs for unauthorized access
3. Enable enhanced logging
4. Contact security team per SECURITY.md
