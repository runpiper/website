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
    strace \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/website /app/website

# Make binary executable and check dependencies
RUN chmod +x /app/website && \
    ls -la /app/website && \
    echo "Binary file type:" && \
    file /app/website && \
    echo "Binary strings (first 20 lines):" && \
    strings /app/website | head -20 && \
    ldd /app/website 2>/dev/null || echo "Binary appears to be statically linked"

# Create a startup script that captures all output
RUN echo '#!/bin/sh\n\
export RUST_BACKTRACE=full\n\
export RUST_LOG=debug\n\
echo "=== Startup Script Starting ==="\n\
echo "Working directory: $(pwd)"\n\
echo "Binary location: /app/website"\n\
echo "Binary exists: $(test -f /app/website && echo yes || echo no)"\n\
echo "Binary executable: $(test -x /app/website && echo yes || echo no)"\n\
echo "Binary size: $(ls -lh /app/website | awk '\''{print $5}'\'')"\n\
echo "PORT env var: ${PORT:-not set}"\n\
echo "=== Testing binary with strace ==="\n\
if command -v strace >/dev/null 2>&1; then\n\
    echo "Running with strace..."\n\
    strace -e trace=write,writev /app/website 2>&1 | head -20\n\
else\n\
    echo "strace not available, running directly..."\n\
    /app/website 2>&1\n\
fi\n\
EXIT_CODE=$?\n\
echo "=== Application Exited ==="\n\
echo "Exit code: $EXIT_CODE"\n\
if [ -f /tmp/rust-main-started.txt ]; then\n\
    echo "Main function was called - file exists:"\n\
    cat /tmp/rust-main-started.txt\n\
else\n\
    echo "Main function was NOT called - file missing"\n\
fi\n\
exit $EXIT_CODE\n\
' > /app/start.sh && chmod +x /app/start.sh

# Expose port
EXPOSE 3000

# Run via wrapper script to see all output
# Using shell form to ensure proper output handling
CMD ["/bin/sh", "-c", "/app/start.sh"]

