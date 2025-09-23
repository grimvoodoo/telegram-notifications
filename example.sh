#!/bin/bash

# Example script for testing telegram-notifications in both CLI and server modes
# 
# Before running this script:
# 1. Create a bot with @BotFather on Telegram
# 2. Get your chat ID (see README.md for instructions)
# 3. Set the environment variables below

# Option 1: Set environment variables (uncomment and edit)
# export TELEGRAM_BOT_TOKEN="1234567890:ABCdefGHIjklMNOpqrSTUvwxyz"
# export TELEGRAM_CHAT_ID="123456789"

# Option 2: Use .env file (recommended)
# cp .env.example .env
# Edit .env with your actual credentials

# Check if environment variables are set
if [[ -z "$TELEGRAM_BOT_TOKEN" ]]; then
    echo "❌ Error: TELEGRAM_BOT_TOKEN environment variable is not set"
    echo "💡 Set it with: export TELEGRAM_BOT_TOKEN=\"your_bot_token_here\""
    exit 1
fi

if [[ -z "$TELEGRAM_CHAT_ID" ]]; then
    echo "❌ Error: TELEGRAM_CHAT_ID environment variable is not set"
    echo "💡 Set it with: export TELEGRAM_CHAT_ID=\"your_chat_id_here\""
    exit 1
fi

echo "🚀 Testing Telegram Bot..."
echo "Bot token: ${TELEGRAM_BOT_TOKEN:0:10}..."
echo "Chat ID: $TELEGRAM_CHAT_ID"
echo ""

# Build the project
echo "🔨 Building project..."
cargo build --release

# Test with default message
echo ""
echo "📤 Sending default test message..."
cargo run --release

# Test with custom message
echo ""
echo "📤 Sending custom message..."
cargo run --release -- --message "🎉 Custom test message from $(date)"

echo ""
echo "✅ CLI mode test completed! Check your Telegram chat for messages."
echo ""

# Test server mode
echo "🌐 Testing HTTP API server mode..."
echo "Starting server in background..."

# Start server in background
cargo run --release -- --server &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Test health endpoint
echo "Testing health endpoint..."
curl -s http://localhost:3000/health | jq '.' || echo "Health check response received"

# Test notification endpoint
echo ""
echo "Testing notification endpoint..."
curl -X POST http://localhost:3000/notify \
  -H "Content-Type: application/json" \
  -d '{"message": "🌐 Hello from HTTP API server! Testing notification endpoint."}' \
  | jq '.' || echo "Notification response received"

# Test with custom formatting
echo ""
echo "Testing with Markdown formatting..."
curl -X POST http://localhost:3000/notify \
  -H "Content-Type: application/json" \
  -d '{"message": "*Bold text* and _italic text_ from API!", "parse_mode": "Markdown"}' \
  | jq '.' || echo "Formatted notification response received"

# Clean up
echo ""
echo "📴 Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo ""
echo "✅ All tests completed! Check your Telegram chat for messages."
echo "💡 To run the server continuously: cargo run --release -- --server"
