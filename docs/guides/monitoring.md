# Monitoring Guide

**Document ID:** GUIDE-MONITOR-001
**Version:** 1.0.0
**Status:** Active
**Owner:** KCM Engineering Team
**Last Reviewed:** 2026-08-04
**Depends on:** KCM_SPECIFICATION

## Purpose

This guide covers metrics collection, dashboards, alerting rules, and log aggregation for KCM deployments.

## Metrics Collection

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'kcm'
    static_configs:
      - targets: ['kcm:8080']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

### Available Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| kcm_facts_inserted | counter | - | Total facts inserted |
| kcm_facts_deleted | counter | - | Total facts deleted |
| kcm_queries_executed | counter | - | Total queries executed |
| kcm_query_duration_seconds | histogram | - | Query latency |
| kcm_wal_writes | counter | - | WAL write operations |
| kcm_cache_hits | counter | - | Cache hit count |
| kcm_cache_misses | counter | - | Cache miss count |
| kcm_bytes_read | counter | - | Total bytes read |
| kcm_bytes_written | counter | - | Total bytes written |
| kcm_errors | counter | - | Total errors |
| kcm_facts_active | gauge | - | Active fact count |

## Dashboards

### Grafana Dashboard

Import the KCM dashboard from `deployment/grafana/kcm-dashboard.json`.

Key panels:
- Query rate (QPS)
- Query latency (p50, p95, p99)
- Insert rate
- Cache hit ratio
- Error rate
- Disk usage

## Alerting Rules

```yaml
groups:
  - name: kcm
    rules:
      - alert: KCMHighErrorRate
        expr: rate(kcm_errors_total[5m]) > 100
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "KCM error rate is high"
          
      - alert: KCMHighLatency
        expr: histogram_quantile(0.99, rate(kcm_query_duration_seconds_bucket[5m])) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "KCM p99 latency is high"
          
      - alert: KCMLowCacheHitRatio
        expr: kcm_cache_hits_total / (kcm_cache_hits_total + kcm_cache_misses_total) < 0.5
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "KCM cache hit ratio is low"
```

## Log Aggregation

### Fluentd Configuration

```yaml
<source>
  @type tail
  path /var/log/kcm/*.log
  pos_file /var/log/kcm/kcm.log.pos
  tag kcm
  <parse>
    @type json
  </parse>
</source>

<match kcm>
  @type elasticsearch
  host elasticsearch
  port 9200
  index_name kcm
</match>
```
