# KCM Deployment Specification

**Document ID:** KCM-DEPLOY-001  
**Version:** 1.0.0  
**Status:** Derived  
**Owner:** Release Readiness (P12)  
**Depends on:** KCM-ARCHDETAIL-001, KCM-SEC-001

---

## 1. Purpose

Defines deployment configurations, container specifications, and Kubernetes manifests for KCM.

---

## 2. Docker Image

### 2.1 Multi-Stage Build

```dockerfile
FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --workspace

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/libkcm_interface.so /usr/local/lib/
EXPOSE 8080
ENV RUST_LOG=info
CMD ["./target/release/kcm-server"]
```

### 2.2 Image Specifications

| Property | Value |
|----------|-------|
| Base image | debian:bookworm-slim |
| Builder image | rust:1.85 |
| Build target | release with LTO |
| Exposed port | 8080 |
| Required env | RUST_LOG |

---

## 3. Docker Compose

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
volumes:
  kcm_data:
    driver: local
```

---

## 4. Kubernetes

### 4.1 StatefulSet

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: kcm-server
spec:
  serviceName: kcm-service
  replicas: 1
  selector:
    matchLabels:
      app: kcm-server
  template:
    metadata:
      labels:
        app: kcm-server
    spec:
      containers:
      - name: kcm-server
        image: kcm:latest
        ports:
        - containerPort: 8080
          name: http
        env:
        - name: RUST_LOG
          value: "info"
        - name: KCM_DATA_PATH
          value: "/data/kcm.db"
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        volumeMounts:
        - name: data
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
      name: data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 100Gi
```

### 4.2 Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: kcm-service
spec:
  selector:
    app: kcm-server
  ports:
  - name: http
    port: 8080
    targetPort: 8080
  type: LoadBalancer
```

### 4.3 PersistentVolumeClaim

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: kcm-data
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Gi
```

---

## 5. Environment Variables

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| RUST_LOG | String | "info" | Log level (error/warn/info/debug/trace) |
| KCM_DATA_PATH | String | — | Path to database file |
| KCM_WAL_PATH | String | — | Path to WAL file |
| KCM_CAPACITY | u64 | 1000000 | Initial schema capacity |
| KCM_WAL_BUFFER | usize | 65536 | WAL buffer threshold (bytes) |

---

## 6. Health Checks

| Endpoint | Method | Expected | Timeout |
|----------|--------|----------|---------|
| /health | GET | 200 OK | 5s |
| /stats | GET | 200 OK | 10s |

### 6.1 Health Response

```json
{
    "status": "healthy|degraded|unhealthy",
    "avg_query_latency_ms": 12.5,
    "cache_hit_ratio": 0.85,
    "insert_error_rate": 0.001,
    "total_queries": 1500,
    "total_inserts": 5000
}
```

---

## 7. Resource Requirements

| Resource | Minimum | Recommended | Notes |
|----------|---------|-------------|-------|
| CPU | 2 cores | 8+ cores | SIMD benefits from wide vectors |
| RAM | 512 MB | 4+ GB | Depends on data volume |
| Disk | 1 GB | 100+ GB | SSD recommended for WAL |
| Network | 10 Mbps | 1 Gbps | For distributed mode |

---

## 8. Constraints

| Constraint | Rationale |
|------------|-----------|
| Persistent storage required | WAL and DB files must survive restarts |
| SSD recommended for WAL | WAL fsync latency critical |
| Minimum 512MB RAM | Schema pre-allocates DenseVec |
| Graceful shutdown required | Flush WAL before exit |

---

## 9. References

- **Depends on:** KCM_ARCHITECTURE (KCM_ARCHITECTURE), KCM_SECURITY_TRUST_SPEC (KCM_SECURITY_TRUST_SPEC)
- **Parent specs:** KCM_SPECIFICATION (KCM_SPECIFICATION)
- **Related:** KCM_RUNTIME_SPEC (KCM_RUNTIME_SPEC), KCM_API_SPEC (KCM_API_SPEC)
