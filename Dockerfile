# Build stage
FROM rust:1.83-slim-bookworm as builder

WORKDIR /usr/src/iggy-dashboard

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    openssl \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install trunk via cargo-binstall
RUN cargo binstall -y trunk

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Copy the entire workspace
COPY . .

# Build frontend
RUN cd frontend && trunk build --release

# Build the server with release profile
RUN cargo build --release --package iggy-dashboard-server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary and frontend files
COPY --from=builder /usr/src/iggy-dashboard/target/release/iggy-dashboard-server /app/
COPY --from=builder /usr/src/iggy-dashboard/frontend/dist /app/frontend/dist

# Create data directory and non-root user
RUN groupadd -r iggy && \
    useradd -r -g iggy -s /bin/false iggy && \
    mkdir -p /data/performance_results && \
    chown -R iggy:iggy /app /data && \
    chmod -R 755 /data/performance_results

# Copy the entrypoint script
COPY docker-entrypoint.sh /app/
RUN chmod +x /app/docker-entrypoint.sh && \
    chown iggy:iggy /app/docker-entrypoint.sh

# Set default environment variables for configuration
ENV HOST=0.0.0.0 \
    PORT=8061 \
    RESULTS_DIR=/data/performance_results

# Set volume for results with proper permissions
VOLUME ["/data/performance_results"]

# Switch to non-root user
USER iggy

# Expose the default port
EXPOSE 8061

# Set the entrypoint script
ENTRYPOINT ["/app/docker-entrypoint.sh"]
