# KCM Deployment

Deployment configurations for Docker, Kubernetes, Helm, and Terraform.

## Contents

| File/Directory | Purpose |
|---------------|---------|
| Dockerfile | Multi-stage container build (rust:1.85 -> debian:bookworm-slim) |
| docker-compose.yml | Local development with single node |
| k8s/ | Kubernetes StatefulSet, Service, PVC |
| helm/ | Helm chart for parameterized deployment |
| terraform/ | Infrastructure as Code modules |

## Quick Start

### Docker

```bash
docker build -t kcm .
docker run -p 8080:8080 -v /data:/data kcm
```

### Docker Compose

```bash
docker-compose up -d
```

### Kubernetes

```bash
kubectl apply -f k8s/
```

### Helm

```bash
helm install kcm helm/
```

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| RUST_LOG | info | Log level |
| KCM_DATA_PATH | /data/kcm.db | Database file path |
| KCM_BIND_ADDR | 0.0.0.0:8080 | Server bind address |

## Health Check

- Endpoint: GET /health
- Port: 8080
- Interval: 10s
- Timeout: 5s
