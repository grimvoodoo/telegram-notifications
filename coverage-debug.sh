#!/bin/bash

set -e

echo "🔍 Coverage Debugging Script"
echo "============================"
echo ""

# Environment info
echo "📋 Environment Info:"
echo "  - Rust version: $(rustc --version)"
echo "  - Cargo version: $(cargo --version)"
echo "  - cargo-llvm-cov version: $(cargo llvm-cov --version)"
echo "  - OS: $(uname -a)"
echo ""

# Clean previous runs
echo "🧹 Cleaning previous runs..."
cargo clean >/dev/null 2>&1
cargo llvm-cov clean >/dev/null 2>&1

# Set environment variables
export TELEGRAM_BOT_TOKEN="test_token:ABCdefGHIjklMNOpqrSTUvwxyz"
export TELEGRAM_CHAT_ID="123456789"
export TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION="true"
export RUST_BACKTRACE=1

echo "✅ Environment variables set"
echo ""

# Try different approaches
echo "🧪 Testing different coverage approaches..."
echo ""

echo "1️⃣  Basic coverage without ignored tests:"
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov-basic.info 2>&1; then
    echo "✅ Basic coverage succeeded"
    echo "   File size: $(stat --format=%s lcov-basic.info 2>/dev/null || echo "N/A") bytes"
else
    echo "❌ Basic coverage failed"
fi
echo ""

echo "2️⃣  Coverage with ignored tests (problematic):"
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov-ignored.info -- --include-ignored 2>&1; then
    echo "✅ Coverage with ignored tests succeeded"
    echo "   File size: $(stat --format=%s lcov-ignored.info 2>/dev/null || echo "N/A") bytes"
else
    echo "❌ Coverage with ignored tests failed"
fi
echo ""

echo "3️⃣  Coverage with timeout and verbose logging:"
if timeout 600 cargo llvm-cov --all-features --workspace --lcov --output-path lcov-timeout.info -- --include-ignored --nocapture 2>&1; then
    echo "✅ Coverage with timeout succeeded"
    echo "   File size: $(stat --format=%s lcov-timeout.info 2>/dev/null || echo "N/A") bytes"
else
    echo "❌ Coverage with timeout failed"
fi
echo ""

echo "4️⃣  Coverage with specific exclusions:"
if cargo llvm-cov --all-features --workspace --lcov --output-path lcov-exclude.info --ignore-filename-regex '(e2e_tests\.rs)' -- --include-ignored 2>&1; then
    echo "✅ Coverage with exclusions succeeded"
    echo "   File size: $(stat --format=%s lcov-exclude.info 2>/dev/null || echo "N/A") bytes"
else
    echo "❌ Coverage with exclusions failed"
fi
echo ""

echo "5️⃣  Run regular tests first, then coverage:"
echo "   Running tests separately..."
if cargo test --all -- --include-ignored >/dev/null 2>&1; then
    echo "✅ Regular tests passed"
    if cargo llvm-cov --all-features --workspace --lcov --output-path lcov-separate.info --no-run >/dev/null 2>&1 && \
       cargo llvm-cov --all-features --workspace --lcov --output-path lcov-separate.info -- --include-ignored 2>&1; then
        echo "✅ Separate coverage succeeded"
        echo "   File size: $(stat --format=%s lcov-separate.info 2>/dev/null || echo "N/A") bytes"
    else
        echo "❌ Separate coverage failed"
    fi
else
    echo "❌ Regular tests failed"
fi
echo ""

echo "📊 Summary of generated files:"
for file in lcov-*.info; do
    if [[ -f "$file" ]]; then
        size=$(stat --format=%s "$file" 2>/dev/null || echo "0")
        lines=$(wc -l < "$file" 2>/dev/null || echo "0")
        echo "  - $file: $size bytes, $lines lines"
    fi
done

echo ""
echo "🎯 Recommendations:"
echo "   If any of the above succeeded, use that approach in CI"
echo "   If all failed, check the specific error messages above"
echo "   E2E tests might be causing issues due to process spawning"