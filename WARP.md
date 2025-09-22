# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust project for telegram notifications. Currently in initial development phase with minimal codebase structure.

## Essential Commands

### Building and Running
```bash
# Build the project
cargo build

# Run the project
cargo run

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
- Consider using popular crates for common functionality:
  - `tokio` for async runtime
  - `serde` for JSON serialization
  - `reqwest` for HTTP requests
  - `clap` for CLI argument parsing
  - `anyhow` or `thiserror` for error handling

## Development Workflow

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