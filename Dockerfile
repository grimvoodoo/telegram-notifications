# Multi-stage build to create minimal production image
FROM rust:1.88-slim as builder

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached unless Cargo.toml changes)
RUN cargo build --release
RUN rm src/main.rs

# Copy actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage - minimal image
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests to Telegram API
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/telegram-notifications /usr/local/bin/telegram-notifications

# Create non-root user for security
RUN useradd --create-home --shell /bin/bash telegram && \
    chown telegram:telegram /usr/local/bin/telegram-notifications

USER telegram

# Expose the default port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Default command runs the server
CMD ["telegram-notifications", "--server"]