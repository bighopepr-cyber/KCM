FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --workspace

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/libkcm_interface.so /usr/local/lib/
EXPOSE 8080
ENV RUST_LOG=info
CMD ["echo", "KCM Library built successfully"]
