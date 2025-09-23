mod api;
mod config;
mod handlers;
mod telegram;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use config::Config;
use dotenv::dotenv;
use handlers::AppState;
use std::sync::Arc;
use telegram::TelegramBot;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (for development)
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "telegram_notifications=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse configuration from command line arguments and environment variables
    let config = Config::from_args_and_env()?;

    // Create the Telegram bot instance
    let bot = TelegramBot::new(config.bot_token.clone());

    // Verify the bot token is valid (skip in test mode)
    let skip_validation = std::env::var("TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION")
        .unwrap_or_default()
        .to_lowercase()
        == "true";

    if !skip_validation {
        info!("🔍 Verifying bot configuration...");
        match bot.get_me().await {
            Ok(response) => {
                if let Some(result) = response.result {
                    if let Some(username) = result["username"].as_str() {
                        info!("✅ Bot verified: @{}", username);
                    } else {
                        info!("✅ Bot verified successfully");
                    }
                }
            }
            Err(e) => {
                tracing::error!("❌ Failed to verify bot: {}", e);
                tracing::error!(
                    "💡 Make sure your bot token is correct and the bot is properly configured with @BotFather"
                );
                return Err(e);
            }
        }
    } else {
        warn!("⚠️  Bot validation skipped (test mode)");
    }

    if config.server {
        // Run as HTTP server
        run_server(config, bot).await
    } else {
        // Run in CLI mode (send single message)
        run_cli_mode(&config, &bot).await
    }
}

async fn run_server(config: config::ConfigResolved, bot: TelegramBot) -> Result<()> {
    let state = Arc::new(AppState {
        bot,
        default_chat_id: config.chat_id.clone(),
    });

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/notify", post(handlers::notify))
        .route("/send", post(handlers::send))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("🚀 Telegram Notifications API server starting...");
    info!("🌐 Listening on http://{}", addr);
    info!("📝 Default chat ID: {}", config.chat_id);
    info!("📄 Available endpoints:");
    info!("    GET  /       - API information");
    info!("    GET  /health - Health check and bot status");
    info!("    POST /notify - Send notification");
    info!("    POST /send   - Send notification (alias)");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_cli_mode(config: &config::ConfigResolved, bot: &TelegramBot) -> Result<()> {
    // Send the test message
    info!("📤 Sending message to chat ID: {}", config.chat_id);
    info!("📝 Message: {}", config.message);

    match bot.send_message(&config.chat_id, &config.message).await {
        Ok(_) => {
            info!("✅ Message sent successfully! 🎉");
            info!("💡 Check your Telegram chat to see the message.");
        }
        Err(e) => {
            tracing::error!("❌ Failed to send message: {}", e);
            warn!("💡 Common issues:");
            warn!("   - Make sure the chat ID is correct");
            warn!("   - If using a group chat, add the bot to the group first");
            warn!("   - If using a private chat, start a conversation with the bot first");
            return Err(e);
        }
    }

    Ok(())
}
