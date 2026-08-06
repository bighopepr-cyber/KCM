# KCM Deployment Specification

**Document ID:** KCM-DEPLOY-001
**Version:** 1.0.0
**Status:** Active
**Owner:** Specification Lock (P4)
**Authority:** P3 (PRD2.md)

---

## 1. Purpose

Defines KCM's deployment configurations: Docker, Kubernetes, Helm, Terraform, and environment variables.

## 2. Docker

### 2.1 Dockerfile

Multi-stage build:
1. **Build stage:** `rust:1.85` — compile release binary
2. **Runtime stage:** `debian:bookworm-slim` — minimal runtime image

### 2.2 Build

```bash
docker build -t kcm:latest .
```

### 2.3 Run

```bash
docker run -d \
  -p 8080:8080 \
  -v kcm_data:/data \
  -e RUST_LOG=info \
  -e KCM_DATA_PATH=/data/kcm.db \
  kcm:latest
```

### 2.4 Docker Compose

```yaml
version: '3.8'
services:
  kcm:
    build: .
    volumes:
      - kcm_data:/data
    environment:
      RUST_LOG: info
      KCM_DATA_PATH: /data/kcm.db
    ports:
      - "8080:8080"
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 10s
      timeout: 5s
      retries: 3
volumes:
  kcm_data:
    driver: local
```

## 3. Kubernetes

### 3.1 StatefulSet

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: kcm
spec:
  serviceName: kcm
  replicas: 1
  selector:
    matchLabels:
      app: kcm
  template:
    metadata:
      labels:
        app: kcm
    spec:
      containers:
      - name: kcm
        image: kcm:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
        - name: KCM_DATA_PATH
          value: "/data/kcm.db"
        volumeMounts:
        - name: kcm-data
          mountPath: /data
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
  volumeClaimTemplates:
  - metadata:
      name: kcm-data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 10Gi
```

### 3.2 Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: kcm
spec:
  selector:
    app: kcm
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
```

## 4. Helm

### 4.1 Install

```bash
helm install kcm deployment/helm/kcm
```

### 4.2 Upgrade

```bash
helm upgrade kcm deployment/helm/kcm
```

### 4.3 Uninstall

```bash
helm uninstall kcm
```

## 5. Terraform

Infrastructure as Code modules for cloud deployment.

## 6. Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |
| `KCM_DATA_PATH` | `/data/kcm.db` | Database file path |
| `KCM_BIND_ADDR` | `0.0.0.0:8080` | Server bind address |

## 7. Health Check

| Property | Value |
|----------|-------|
| Endpoint | `GET /health` |
| Port | 8080 |
| Interval | 10s |
| Timeout | 5s |
| Healthy | error_rate < 5%, cache_hit_ratio > 50% |
| Degraded | latency > 100ms OR cache_hit_ratio < 50% |
| Unhealthy | error_rate ≥ 5% |

## 8. Resource Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| CPU | 1 core | 4 cores |
| Memory | 256 MB | 2 GB |
| Storage | 1 GB | 100 GB |
| Network | 10 Mbps | 100 Mbps |

## 9. Monitoring

### 9.1 Prometheus

- Metrics endpoint: `GET /metrics`
- Format: Prometheus text exposition format
- Scrape interval: 15s recommended

### 9.2 Grafana

- Dashboard templates in `deployment/grafana/`
- Pre-built panels for all 14 metrics

## 10. Security

- Run as non-root user in container
- Enable audit logging in production
- Use encrypted connections for network communication
- Follow principle of least privilege for database access

## 11. References

- **Implements:** PRD2.md (Runtime, Deployment)
- **Depends on:** KCM_RUNTIME_SPEC, KCM_API_SPEC
- **Related:** KCM_SECURITY_TRUST_SPEC, KCM_PERFORMANCE_SPEC
