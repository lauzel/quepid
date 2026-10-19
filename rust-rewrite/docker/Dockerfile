# Quepid Rust Dockerfile
#
# Multi-stage build for production and development

# Build stage
FROM rust:1.80-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy workspace files
COPY rust-rewrite/Cargo.toml rust-rewrite/Cargo.toml
COPY rust-rewrite/rust-toolchain rust-rewrite/rust-toolchain

# Copy all crate files
COPY rust-rewrite/crates/quepid-core rust-rewrite/crates/quepid-core
COPY rust-rewrite/crates/quepid-infra rust-rewrite/crates/quepid-infra
COPY rust-rewrite/crates/quepid-api rust-rewrite/crates/quepid-api
COPY rust-rewrite/crates/quepid-workers rust-rewrite/crates/quepid-workers
COPY rust-rewrite/crates/quepid-websocket rust-rewrite/crates/quepid-websocket
COPY rust-rewrite/crates/quepid-cli rust-rewrite/crates/quepid-cli

# Build in release mode
RUN cd rust-rewrite && \
    cargo build --release --all

# Production stage
FROM debian:12-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/rust-rewrite/target/release/quepid-api /usr/local/bin/quepid-api
COPY --from=builder /app/rust-rewrite/target/release/quepid-workers /usr/local/bin/quepid-workers
COPY --from=builder /app/rust-rewrite/target/release/quepid-websocket /usr/local/bin/quepid-websocket

# Create config directory
RUN mkdir -p /etc/quepid

# Copy configuration
COPY rust-rewrite/docker/config.toml /etc/quepid/config.toml

# Expose ports
EXPOSE 3000

# Set environment variables
ENV QUEPID_ENV=production
ENV RUST_LOG=info
ENV DATABASE_URL=mysql://quepid:quepid@mysql/quepid_production

# Run the API server
CMD ["quepid-api"]
