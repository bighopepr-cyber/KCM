FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin kcm-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/kcm-server /usr/local/bin/
EXPOSE 8080
ENV RUST_LOG=info
CMD ["kcm-server"]
