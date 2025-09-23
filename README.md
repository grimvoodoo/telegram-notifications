# Telegram Notifications Service

A fast, lightweight Rust application for sending notifications via Telegram. Supports both CLI mode and HTTP API server mode, making it perfect for integrating with various applications, CI/CD pipelines, monitoring systems, microservices, or any automation that needs to send alerts.

## Features

### 💻 Dual Mode Operation
- **CLI Mode**: Send single notifications from command line
- **HTTP API Server**: RESTful API for integration with other applications

### 🚀 Performance & Reliability
- Fast and lightweight Rust implementation
- Built-in bot verification and health checks
- Comprehensive error handling with detailed error messages
- Request tracing and structured logging

### 🔒 Security & Configuration  
- Secure token handling via environment variables or `.env` files
- Support for custom chat IDs per request (API mode)
- CORS support for browser integration

### 📝 Message Features
- Support for Markdown and HTML formatting
- Silent notifications (disable_notification)
- Message length validation
- Response includes Telegram message ID

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- A Telegram bot token (see setup instructions below)

## Quick Start

### 1. Create a Telegram Bot

1. Open Telegram and search for `@BotFather`
2. Start a chat and send `/newbot`
3. Follow the prompts to name your bot
4. Save the bot token provided by BotFather (keep it secure!)

### 2. Get Your Chat ID

**For personal messages:**
1. Start a conversation with your bot by searching for its username
2. Send any message to the bot
3. Visit: `https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates`
4. Look for `"chat":{"id":YOUR_CHAT_ID}` in the response

**For group messages:**
1. Add your bot to the group
2. Send a message mentioning the bot (e.g., `@your_bot_name test`)
3. Visit: `https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates`
4. Look for the group chat ID (will be a negative number)

### 3. Build and Run

```bash
# Clone or download this repository
cd telegram-notifications

# Build the project
cargo build --release

# Option 1: Use .env file (recommended for development)
cp .env.example .env
# Edit .env with your actual bot token and chat ID

# Option 2: Set environment variables directly
export TELEGRAM_BOT_TOKEN="your_bot_token_here"
export TELEGRAM_CHAT_ID="your_chat_id_here"

# Run with default test message
cargo run

# Or run the compiled binary
./target/release/telegram-notifications
```

## Usage

### CLI Mode

#### Environment Variables (Recommended)

**Option 1: Use .env file (best for development)**
```bash
# Copy and edit the example file
cp .env.example .env
# Edit .env with your actual credentials
cargo run
```

**Option 2: Export environment variables**
```bash
export TELEGRAM_BOT_TOKEN="1234567890:ABCdefGHIjklMNOpqrSTUvwxyz"
export TELEGRAM_CHAT_ID="123456789"
cargo run
```

### Command Line Arguments

```bash
# Send a custom message
cargo run -- --message "Hello from my application! 🚀"

# Specify bot token and chat ID via command line
cargo run -- --bot-token "1234567890:ABC..." --chat-id "123456789" --message "Test message"

# View help
cargo run -- --help
```

### Usage Examples

**Basic notification:**
```bash
cargo run -- --message "Deployment completed successfully ✅"
```

**Server monitoring alert:**
```bash
cargo run -- --message "⚠️ *Alert*: CPU usage exceeded 80% on server production-01"
```

**CI/CD Integration:**
```bash
# In your CI/CD pipeline
./telegram-notifications --message "🎉 Build #$BUILD_NUMBER completed successfully"
```

### HTTP API Server Mode

Run the application as an HTTP server to receive API calls from other applications:

```bash
# Start the server (will run on http://0.0.0.0:3000 by default)
cargo run -- --server

# Or specify custom port and host
cargo run -- --server --port 8080 --host 127.0.0.1

# Using environment variables
export PORT=8080
cargo run -- --server
```

#### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | API information and available endpoints |
| `GET` | `/health` | Health check and bot verification status |
| `POST` | `/notify` | Send a notification message |
| `POST` | `/send` | Send a notification message (alias for `/notify`) |

#### Send Notification

**POST** `/notify` or `/send`

**Request Body:**
```json
{
  "message": "Your notification message here! 🚀",
  "chat_id": "123456789",           // Optional: override default chat_id
  "parse_mode": "Markdown",          // Optional: "Markdown", "HTML", or null
  "disable_notification": false     // Optional: send silent notification
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Notification sent successfully",
  "telegram_message_id": 42
}
```

**Response (Error):**
```json
{
  "success": false,
  "error": "Message cannot be empty",
  "code": "EMPTY_MESSAGE"
}
```

#### Health Check

**GET** `/health`

**Response:**
```json
{
  "status": "healthy",
  "service": "telegram-notifications",
  "version": "0.1.0",
  "bot_verified": true,
  "bot_username": "your_bot_name"
}
```

#### API Examples

**Using curl:**
```bash
# Send a basic notification
curl -X POST http://localhost:3000/notify \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from API! 👋"}'

# Send with custom formatting and chat ID
curl -X POST http://localhost:3000/notify \
  -H "Content-Type: application/json" \
  -d '{
    "message": "*Alert*: System CPU usage is high!",
    "chat_id": "987654321",
    "parse_mode": "Markdown"
  }'

# Send silent notification
curl -X POST http://localhost:3000/notify \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Background task completed",
    "disable_notification": true
  }'

# Check health
curl http://localhost:3000/health
```

**Using Python:**
```python
import requests

# Send notification
response = requests.post('http://localhost:3000/notify', json={
    'message': 'Deployment completed successfully! 🎉',
    'parse_mode': 'Markdown'
})
print(response.json())
```

**Using JavaScript:**
```javascript
// Send notification
fetch('http://localhost:3000/notify', {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    message: 'User registration: New user signed up! 👤',
    parse_mode: 'Markdown'
  })
})
.then(response => response.json())
.then(data => console.log(data));
```

## Integration Examples

### Shell Script Integration
```bash
#!/bin/bash
# deploy.sh

# Your deployment logic here
if deploy_app; then
    ./telegram-notifications --message "✅ Deployment successful"
else
    ./telegram-notifications --message "❌ Deployment failed"
fi
```

### GitHub Actions
```yaml
# .github/workflows/notify.yml
- name: Notify Telegram
  run: |
    ./telegram-notifications --message "🚀 Deployment to production completed"
  env:
    TELEGRAM_BOT_TOKEN: ${{ secrets.TELEGRAM_BOT_TOKEN }}
    TELEGRAM_CHAT_ID: ${{ secrets.TELEGRAM_CHAT_ID }}
```

### Cron Job Monitoring
```bash
# Add to crontab
0 */6 * * * /path/to/telegram-notifications --message "🤖 Backup completed at $(date)"
```

## Deployment

### Docker Deployment

Create a `Dockerfile`:
```dockerfile
FROM rust:1.88-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/telegram-notifications /usr/local/bin/
EXPOSE 3000
CMD ["telegram-notifications", "--server"]
```

Build and run:
```bash
# Build the image
docker build -t telegram-notifications .

# Run the server
docker run -d \
  --name telegram-notifications \
  -p 3000:3000 \
  -e TELEGRAM_BOT_TOKEN="your_bot_token_here" \
  -e TELEGRAM_CHAT_ID="your_chat_id_here" \
  telegram-notifications
```

### Systemd Service

Create `/etc/systemd/system/telegram-notifications.service`:
```ini
[Unit]
Description=Telegram Notifications API
After=network.target

[Service]
Type=simple
User=telegram-notifications
Group=telegram-notifications
WorkingDirectory=/opt/telegram-notifications
ExecStart=/opt/telegram-notifications/telegram-notifications --server
Restart=always
RestartSec=5
Environment=TELEGRAM_BOT_TOKEN=your_bot_token_here
Environment=TELEGRAM_CHAT_ID=your_chat_id_here
Environment=RUST_LOG=telegram_notifications=info

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable telegram-notifications
sudo systemctl start telegram-notifications
```

### Microservice Integration Examples

**With Express.js (Node.js):**
```javascript
const express = require('express');
const axios = require('axios');

const app = express();
const TELEGRAM_API = 'http://localhost:3000';

app.post('/notify-user-signup', async (req, res) => {
  try {
    await axios.post(`${TELEGRAM_API}/notify`, {
      message: `👤 New user registered: ${req.body.username}`,
      parse_mode: 'Markdown'
    });
    res.json({ success: true });
  } catch (error) {
    res.status(500).json({ error: 'Notification failed' });
  }
});
```

**With FastAPI (Python):**
```python
from fastapi import FastAPI
import httpx

app = FastAPI()
TELEGRAM_API = "http://localhost:3000"

@app.post("/notify-deployment")
async def notify_deployment(deployment_info: dict):
    async with httpx.AsyncClient() as client:
        response = await client.post(
            f"{TELEGRAM_API}/notify",
            json={
                "message": f"🚀 Deployment completed: {deployment_info['service']}",
                "parse_mode": "Markdown"
            }
        )
    return response.json()
```

**With Spring Boot (Java):**
```java
@RestController
public class NotificationController {
    
    @Autowired
    private WebClient webClient;
    
    @PostMapping("/notify-error")
    public Mono<String> notifyError(@RequestBody ErrorInfo error) {
        return webClient.post()
            .uri("http://localhost:3000/notify")
            .bodyValue(Map.of(
                "message", "⚠️ Error in " + error.getService() + ": " + error.getMessage(),
                "parse_mode", "Markdown"
            ))
            .retrieve()
            .bodyToMono(String.class);
    }
}
```

## Configuration

| Environment Variable | Command Line Flag | Description | Required |
|---------------------|-------------------|-------------|-----------|
| `TELEGRAM_BOT_TOKEN` | `--bot-token` | Your bot token from BotFather | Yes |
| `TELEGRAM_CHAT_ID` | `--chat-id` | Target chat ID for messages | Yes |
| N/A | `--message` | Custom message to send | No (default provided) |

## Troubleshooting

### Common Error Messages

**"Bot token is required"**
- Make sure you've set the `TELEGRAM_BOT_TOKEN` environment variable or used the `--bot-token` flag

**"Chat ID is required"**
- Make sure you've set the `TELEGRAM_CHAT_ID` environment variable or used the `--chat-id` flag

**"Failed to verify bot"**
- Double-check your bot token
- Ensure the bot hasn't been deleted in BotFather

**"Failed to send message"**
- For private chats: Make sure you've started a conversation with the bot first
- For group chats: Make sure the bot has been added to the group
- Verify the chat ID is correct (group IDs are negative numbers)

### Getting Help

Run the application with `--help` to see all available options:
```bash
cargo run -- --help
```

## Security Notes

- Never commit bot tokens to version control
- Use environment variables for production deployments
- Consider using secrets management systems for sensitive deployments
- The bot token provides full access to your bot - treat it like a password

## Development

See [WARP.md](WARP.md) for development guidelines and common commands.

## License

This project is open source and available under the MIT License.