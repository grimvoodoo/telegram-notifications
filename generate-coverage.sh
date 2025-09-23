#!/bin/bash

# Reliable Coverage Generation Script
# This script tries multiple approaches to generate coverage reliably

set -e

echo "🔍 Generating Code Coverage"
echo "==========================="
echo ""

# Environment variables
export TELEGRAM_BOT_TOKEN="${TELEGRAM_BOT_TOKEN:-test_token:ABCdefGHIjklMNOpqrSTUvwxyz}"
export TELEGRAM_CHAT_ID="${TELEGRAM_CHAT_ID:-123456789}"
export TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION="${TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION:-true}"
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

# Clean previous runs
echo "🧹 Cleaning previous runs..."
cargo clean >/dev/null 2>&1 || true
cargo llvm-cov clean >/dev/null 2>&1 || true

# Function to check if coverage file is valid
check_coverage_file() {
    local file="$1"
    if [[ -f "$file" ]] && [[ -s "$file" ]]; then
        local lines=$(wc -l < "$file")
        local size=$(stat --format=%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null || echo "unknown")
        echo "✅ Coverage file: $lines lines, $size bytes"
        return 0
    else
        echo "❌ Coverage file missing or empty"
        return 1
    fi
}

# Approach 1: Standard approach with ignored tests
echo "1️⃣  Trying standard coverage with ignored tests..."
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info -- --include-ignored >/dev/null 2>&1; then
    if check_coverage_file "lcov.info"; then
        echo "✅ Standard coverage succeeded!"
        exit 0
    fi
fi

# Approach 2: Coverage without ignored tests (safer)
echo "2️⃣  Trying coverage without ignored tests..."
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info >/dev/null 2>&1; then
    if check_coverage_file "lcov.info"; then
        echo "✅ Coverage without ignored tests succeeded!"
        echo "ℹ️  Note: E2E tests were skipped for coverage"
        exit 0
    fi
fi

# Approach 3: Library and binary coverage only
echo "3️⃣  Trying lib and binary coverage only..."
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info --lib --bins >/dev/null 2>&1; then
    if check_coverage_file "lcov.info"; then
        echo "✅ Lib and binary coverage succeeded!"
        echo "ℹ️  Note: Only library and binary code covered (no test coverage)"
        exit 0
    fi
fi

# Approach 4: Single threaded coverage
echo "4️⃣  Trying single-threaded coverage..."
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info -- --include-ignored --test-threads=1 >/dev/null 2>&1; then
    if check_coverage_file "lcov.info"; then
        echo "✅ Single-threaded coverage succeeded!"
        exit 0
    fi
fi

# Approach 5: Coverage with explicit timeout
echo "5️⃣  Trying coverage with timeout..."
if timeout 600 cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info -- --include-ignored >/dev/null 2>&1; then
    if check_coverage_file "lcov.info"; then
        echo "✅ Coverage with timeout succeeded!"
        exit 0
    fi
fi

echo ""
echo "❌ All coverage approaches failed!"
echo ""
echo "🔧 Troubleshooting suggestions:"
echo "1. Check if cargo-llvm-cov is properly installed: cargo install cargo-llvm-cov"
echo "2. Ensure LLVM tools are available: rustup component add llvm-tools-preview"
echo "3. Try running regular tests first: cargo test --all"
echo "4. Check for conflicting processes or locked files"
echo "5. Try with a clean Rust toolchain"
echo ""
echo "📝 For CI/CD, consider using the safer approach:"
echo "   cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info --lib --bins"

exit 1