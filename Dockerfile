# Multi-stage build to create minimal production image
FROM rust:1 as builder

# Install musl tools
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# Install musl target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached unless Cargo.toml changes)
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm src/main.rs

# Copy actual source code
COPY src ./src

# Build the application as static binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage - scratch image for minimal footprint
FROM scratch

# Copy CA certificates from builder stage for HTTPS requests
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the statically-linked binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/telegram-notifications /telegram-notifications

# Expose the default port
EXPOSE 3000

# Set the binary as entrypoint so arguments can be passed
ENTRYPOINT ["/telegram-notifications"]

# Default arguments (can be overridden)
CMD ["--server"]
