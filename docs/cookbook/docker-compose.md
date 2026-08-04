# Docker Compose Recipes

## Development Environment

```yaml
version: '3.8'
services:
  kcm:
    build:
      context: ..
      dockerfile: deployment/Dockerfile
    ports:
      - "8080:8080"
      - "50051:50051"
    volumes:
      - kcm_data:/data
      - ../logs:/var/log/kcm
    environment:
      - RUST_LOG=debug
      - KCM_DATA_PATH=/data/kcm.db
      - KCM_BIND_ADDR=0.0.0.0:8080
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:8080/health"]
      interval: 10s
      timeout: 5s
      retries: 3
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    depends_on:
      - kcm

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana

volumes:
  kcm_data:
  grafana_data:
```

## Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'kcm'
    static_configs:
      - targets: ['kcm:8080']
    metrics_path: '/metrics'
```

## Usage

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f kcm

# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```
