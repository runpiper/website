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

# Force rebuild by touching source files and cleaning old artifacts
RUN touch src/main.rs && \
    rm -f target/release/website target/release/deps/website-* && \
    cargo build --release && \
    ls -lh target/release/website && \
    echo "Verifying binary contains our code..." && \
    strings target/release/website | grep -i "RUST_APP" | head -5 || echo "RUST_APP not found in binary - this is a problem!"

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

# Copy static assets
COPY static /app/static

# Make binary executable and check dependencies
RUN chmod +x /app/website && \
    ls -la /app/website && \
    echo "Binary file type:" && \
    file /app/website && \
    echo "Binary strings (first 20 lines):" && \
    strings /app/website | head -20 && \
    ldd /app/website 2>/dev/null || echo "Binary appears to be statically linked"

# No startup script needed - run binary directly

# Expose port
EXPOSE 3000

# Run the binary directly
CMD ["/app/website"]

