# kcm-server

HTTP (actix-web) and gRPC (tonic) server binaries for KCM.

## Purpose

Provides production server binaries that expose KCM's REST API and gRPC service over the network.

## Binaries

| Binary | Source | Description |
|--------|--------|-------------|
| `kcm-server` | `src/main.rs` | HTTP REST API server (actix-web) |
| `kcm-grpc` | `src/grpc_main.rs` | gRPC server (tonic) |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-runtime` | Database operations |
| `kcm-interface` | API handlers |
| `actix-web` | HTTP framework |
| `tonic` / `prost` | gRPC framework |
| `tokio` | Async runtime |
| `env_logger` | Logging |
| `serde` / `serde_json` | JSON serialization |

## HTTP Server

```bash
# Start HTTP server
cargo run --bin kcm-server

# Custom configuration
KCM_BIND_ADDR=0.0.0.0:9090 KCM_DATA_PATH=/data/kcm.db cargo run --bin kcm-server
```

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /health | Health check |
| POST | /api/facts | Insert fact |
| DELETE | /api/facts | Delete fact |
| GET | /api/facts | Query facts |
| GET | /api/metrics | Performance metrics |
| POST | /api/backup | Backup database |

## gRPC Server

```bash
# Start gRPC server
cargo run --bin kcm-grpc
```

### Proto Definition

```protobuf
service KcmService {
    rpc Insert(InsertRequest) returns (InsertResponse);
    rpc Delete(DeleteRequest) returns (DeleteResponse);
    rpc Query(QueryRequest) returns (stream QueryResponse);
    rpc HealthCheck(HealthRequest) returns (HealthResponse);
    rpc Metrics(MetricsRequest) returns (MetricsResponse);
}
```

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| RUST_LOG | info | Log level |
| KCM_DATA_PATH | /data/kcm.db | Database file path |
| KCM_BIND_ADDR | 0.0.0.0:8080 | Server bind address |
| KCM_GRPC_ADDR | 0.0.0.0:9090 | gRPC bind address |
