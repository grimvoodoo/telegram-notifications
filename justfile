# Telegram Notifications Development Tasks
# Usage: just <task>

# Default recipe shows available tasks
default:
    @just --list

# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run the application in CLI mode
run *ARGS:
    cargo run -- {{ARGS}}

# Run the application in server mode
serve PORT="3000":
    cargo run -- --server --port {{PORT}}

# Run all tests
test:
    cargo test

# Run unit tests only
test-unit:
    cargo test --lib

# Run end-to-end tests (requires server startup)
test-e2e:
    cargo test --test e2e_tests -- --ignored

# Run tests with coverage
test-coverage:
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Format code
fmt:
    cargo fmt

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy issues automatically
lint-fix:
    cargo clippy --all-targets --all-features --fix -- -D warnings

# Check the project without building
check:
    cargo check

# Run security audit
audit:
    cargo audit

# Clean build artifacts
clean:
    cargo clean

# Generate documentation
docs:
    cargo doc --open

# Run example script
example:
    ./example.sh

# Build Docker image
docker-build:
    docker build -t telegram-notifications .

# Run Docker container
docker-run PORT="3000":
    docker run -p {{PORT}}:3000 \
        -e TELEGRAM_BOT_TOKEN="$TELEGRAM_BOT_TOKEN" \
        -e TELEGRAM_CHAT_ID="$TELEGRAM_CHAT_ID" \
        telegram-notifications

# Start development server with hot reload
dev:
    cargo watch -x "run -- --server"

# Run all quality checks (format, lint, test)
ci:
    just fmt-check
    just lint
    just test
    just build-release

# Set up development environment
setup:
    @echo "Setting up development environment..."
    cp .env.example .env
    @echo "✅ Created .env file from template"
    @echo "📝 Edit .env with your actual bot credentials"
    @echo "🚀 Run 'just serve' to start the development server"

# Release build and test
release:
    just ci
    just build-release
    @echo "✅ Release build complete: target/release/telegram-notifications"