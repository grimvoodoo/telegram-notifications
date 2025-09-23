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

# Runtime stage - scratch image for minimal footprint
FROM scratch

# Copy CA certificates from builder stage
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/telegram-notifications /telegram-notifications

# Expose the default port
EXPOSE 3000

# Default command runs the server
CMD ["/telegram-notifications", "--server"]
