# 🦾 Calclaw - Docker Image
# Multi-stage build for production

# Stage 1: Builder
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    git \
    curl

# Create app directory
WORKDIR /app

# Copy source code
COPY . .

# Build the application
RUN cargo build --release --locked

# Stage 2: Runtime
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    openssl \
    libgcc \
    tzdata \
    curl

# Create non-root user
RUN addgroup -g 1000 calclaw && \
    adduser -u 1000 -G calclaw -s /bin/sh -D calclaw

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/calclaw /usr/local/bin/calclaw

# Copy configuration files
COPY --chown=calclaw:calclaw config/ /app/config/
COPY --chown=calclaw:calclaw scripts/ /app/scripts/

# Create necessary directories
RUN mkdir -p /app/data /app/logs /app/tmp && \
    chown -R calclaw:calclaw /app

# Switch to non-root user
USER calclaw

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/api/health || exit 1

# Expose port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV PORT=3000
ENV HOST=0.0.0.0
ENV DATA_DIR=/app/data
ENV LOG_DIR=/app/logs
ENV CONFIG_DIR=/app/config

# Run the application
CMD ["calclaw"]