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
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/website /app/website

# Copy templates directory (needed by Askama at runtime)
COPY --from=builder /app/templates ./templates

# Make binary executable
RUN chmod +x /app/website

# Expose port
EXPOSE 3000

# Run the application
CMD ["./website"]

