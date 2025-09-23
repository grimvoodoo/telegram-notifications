# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust project for telegram notifications that supports both CLI mode and HTTP API server mode. The application can send single notifications via command line or run as a web service to receive API calls from other applications.

## Essential Commands

### Building and Running
```bash
# Build the project
cargo build

# Run the project in CLI mode (default)
cargo run

# Run as HTTP API server
cargo run -- --server

# Run server on custom port
cargo run -- --server --port 8080

# Build optimized release version
cargo build --release

# Check code without building
cargo check
```

### Testing
```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific file/module
cargo test module_name::

# Test API endpoints (requires server running)
curl http://localhost:3000/health
curl -X POST http://localhost:3000/notify -H "Content-Type: application/json" -d '{"message":"Test"}'

# Test with example script
./example.sh
```

### Development Tools
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security advisories
cargo audit  # (requires cargo-audit: cargo install cargo-audit)

# Generate documentation
cargo doc --open
```

### Testing

The project includes comprehensive unit tests and end-to-end tests:

```bash
# Run all unit tests
cargo test --lib

# Run CLI tests (no server required)
cargo test --test e2e_tests

# Run end-to-end server tests (requires validation skip)
cargo test --test e2e_tests -- --ignored

# Run all tests with coverage
cargo llvm-cov --all-features --workspace
```

#### Test Mode

For testing the server functionality without a valid Telegram bot token, set the `TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION` environment variable:

```bash
# Set test mode to skip bot validation
export TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION=true

# Or use the test environment file
cp .env.test .env
```

In test mode:
- Bot token validation is skipped during startup
- Health endpoint returns `bot_verified: false` but still responds successfully
- Notification endpoints return mock success responses without actually calling the Telegram API
- Perfect for automated testing, CI/CD, and development without affecting real Telegram chats

## Code Architecture

### Current Structure
- `src/main.rs`: Main entry point with basic "Hello, world!" implementation
- `Cargo.toml`: Project configuration using Rust 2024 edition
- Currently no dependencies defined

### Expected Development Direction
Based on the project name "telegram-notifications", this likely will evolve into:
- Telegram Bot API integration
- Notification handling and dispatching
- Configuration management for bot tokens and chat IDs
- Message formatting and templating
- Error handling and retry logic

### Rust-Specific Guidelines
- Use `Cargo.toml` to manage dependencies
- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types)
- Leverage Rust's ownership system and error handling with `Result<T, E>`
- Environment variable loading uses `dotenv` for development convenience:
  - `dotenv().ok()` is called at startup to load `.env` files in development
  - Production deployments should use real environment variables
  - The pattern follows the same approach as the foxy-fabrications project
- Consider using popular crates for common functionality:
  - `tokio` for async runtime
  - `serde` for JSON serialization
  - `reqwest` for HTTP requests
  - `clap` for CLI argument parsing
  - `anyhow` or `thiserror` for error handling
  - `dotenv` for development environment management

## Development Workflow

### Environment Configuration

The project uses `dotenv` for loading environment variables during development:

```bash
# Copy the example environment file
cp .env.example .env
# Edit .env with your actual credentials
```

**Important:** The `.env` file is gitignored and should never be committed. For production deployments, use actual environment variables instead of `.env` files.

### Adding Dependencies
Add dependencies to `Cargo.toml` under `[dependencies]` section, then run:
```bash
cargo build
```

### Code Organization
- Keep main business logic in separate modules under `src/`
- Use `src/lib.rs` for library code if the project grows
- Consider `src/bin/` for multiple binaries
- Use `tests/` directory for integration tests

### Git Workflow
- This is a fresh repository with no commits yet
- Standard Rust `.gitignore` is already configured to ignore `/target`