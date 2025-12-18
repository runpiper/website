# Build stage
FROM rust:slim as builder

WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release
RUN rm src/main.rs

# Copy actual source code
COPY src ./src
COPY templates ./templates

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libc6 \
    libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/website /app/website

# Make binary executable and check dependencies
RUN chmod +x /app/website && \
    ls -la /app/website && \
    ldd /app/website 2>/dev/null || echo "Binary appears to be statically linked"

# Create a startup script that captures all output
RUN echo '#!/bin/sh\n\
set -e\n\
export RUST_BACKTRACE=full\n\
export RUST_LOG=debug\n\
echo "=== Startup Script Starting ==="\n\
echo "Working directory: $(pwd)"\n\
echo "Binary location: /app/website"\n\
echo "Binary exists: $(test -f /app/website && echo yes || echo no)"\n\
echo "Binary executable: $(test -x /app/website && echo yes || echo no)"\n\
echo "PORT env var: ${PORT:-not set}"\n\
echo "=== Starting Application ==="\n\
exec /app/website\n\
' > /app/start.sh && chmod +x /app/start.sh

# Expose port
EXPOSE 3000

# Run via wrapper script to see all output
CMD ["/app/start.sh"]

