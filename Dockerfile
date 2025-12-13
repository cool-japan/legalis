# Multi-stage Dockerfile for Legalis-RS
# Builds optimized production binaries for CLI and API server

# =============================================================================
# Stage 1: Builder - Compile Rust binaries
# =============================================================================
FROM rust:1.83-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo

# Copy all crates
COPY crates ./crates
COPY jurisdictions ./jurisdictions
COPY examples ./examples

# Build release binaries with optimizations
RUN cargo build --release \
    --bin legalis \
    --bin legalis-api-server \
    --features "z3-solver"

# Strip binaries to reduce size
RUN strip /app/target/release/legalis
RUN strip /app/target/release/legalis-api-server

# =============================================================================
# Stage 2: Runtime - Minimal production image
# =============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 -s /bin/bash legalis

# Create app directory
WORKDIR /app

# Copy binaries from builder
COPY --from=builder /app/target/release/legalis /usr/local/bin/
COPY --from=builder /app/target/release/legalis-api-server /usr/local/bin/

# Copy example files and documentation (optional)
COPY --from=builder /app/examples /app/examples
COPY README.md LICENSE ./

# Set ownership
RUN chown -R legalis:legalis /app

# Switch to non-root user
USER legalis

# Expose API server port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV LEGALIS_HOST=0.0.0.0
ENV LEGALIS_PORT=3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Default command: run API server
CMD ["legalis-api-server"]

# Labels for metadata
LABEL org.opencontainers.image.title="Legalis-RS"
LABEL org.opencontainers.image.description="Formal specification and verification framework for legal statutes"
LABEL org.opencontainers.image.version="0.2.0"
LABEL org.opencontainers.image.authors="Legalis-RS Contributors"
LABEL org.opencontainers.image.url="https://github.com/yourusername/legalis-rs"
LABEL org.opencontainers.image.documentation="https://github.com/yourusername/legalis-rs/blob/main/README.md"
LABEL org.opencontainers.image.source="https://github.com/yourusername/legalis-rs"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
