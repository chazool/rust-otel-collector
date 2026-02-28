FROM rust:1.93 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS final

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust-otel-collector .

RUN chmod +x /app/rust-otel-collector

EXPOSE 8080
CMD ["./rust-otel-collector"]
