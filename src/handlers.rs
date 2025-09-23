use crate::api::{
    ErrorResponse, HealthResponse, InfoResponse, SendNotificationRequest, SendNotificationResponse,
};
use crate::telegram::TelegramBot;
use axum::{Json as JsonExtractor, extract::State, http::StatusCode, response::Json};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info, warn};

pub struct AppState {
    pub bot: TelegramBot,
    pub default_chat_id: String,
}

/// GET / - API information
pub async fn root() -> Json<InfoResponse> {
    Json(InfoResponse::new())
}

/// GET /health - Health check and bot verification
pub async fn health(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthResponse>, (StatusCode, Json<ErrorResponse>)> {
    info("🔍 Health check requested");

    // Check if we're in test mode (validation was skipped)
    let skip_validation = std::env::var("TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION")
        .unwrap_or_default()
        .to_lowercase()
        == "true";

    if skip_validation {
        info("⚠️  Health check in test mode (bot validation skipped)");
        Ok(Json(HealthResponse {
            status: "healthy".to_string(),
            service: "telegram-notifications".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            bot_verified: false,
            bot_username: Some("test-bot".to_string()),
        }))
    } else {
        match state.bot.get_me().await {
            Ok(response) => {
                let bot_username = response
                    .result
                    .as_ref()
                    .and_then(|result| result.get("username"))
                    .and_then(|username| username.as_str())
                    .map(|s| s.to_string());

                info("✅ Health check passed - bot verified");
                Ok(Json(HealthResponse {
                    status: "healthy".to_string(),
                    service: "telegram-notifications".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    bot_verified: true,
                    bot_username,
                }))
            }
            Err(e) => {
                error!("❌ Health check failed - bot verification error: {}", e);
                Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ErrorResponse::with_code(
                        "Bot verification failed".to_string(),
                        "BOT_VERIFICATION_FAILED".to_string(),
                    )),
                ))
            }
        }
    }
}

/// POST /notify - Send notification
pub async fn notify(
    State(state): State<Arc<AppState>>,
    JsonExtractor(request): JsonExtractor<SendNotificationRequest>,
) -> Result<Json<SendNotificationResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(
        "📤 Notification request received: {}",
        request.message.chars().take(50).collect::<String>()
    );

    // Validate message
    if request.message.is_empty() {
        warn!("⚠️ Empty message in notification request");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::with_code(
                "Message cannot be empty".to_string(),
                "EMPTY_MESSAGE".to_string(),
            )),
        ));
    }

    // Use custom chat_id or default
    let chat_id = request.chat_id.as_ref().unwrap_or(&state.default_chat_id);

    // Check if we're in test mode
    let skip_validation = std::env::var("TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION")
        .unwrap_or_default()
        .to_lowercase()
        == "true";

    if skip_validation {
        info!("⚠️  Test mode: Simulating message send to chat {}", chat_id);
        Ok(Json(SendNotificationResponse {
            success: true,
            message: "Notification sent successfully (test mode)".to_string(),
            telegram_message_id: Some(42), // Mock message ID
        }))
    } else {
        // Send the message
        match state
            .bot
            .send_message_advanced(
                chat_id,
                &request.message,
                request.parse_mode.as_deref(),
                request.disable_notification.unwrap_or(false),
            )
            .await
        {
            Ok(response) => {
                let message_id = extract_message_id(&response.result);
                info!("✅ Notification sent successfully to chat {}", chat_id);

                Ok(Json(SendNotificationResponse {
                    success: true,
                    message: "Notification sent successfully".to_string(),
                    telegram_message_id: message_id,
                }))
            }
            Err(e) => {
                error!("❌ Failed to send notification: {}", e);
                Err((
                    StatusCode::BAD_GATEWAY,
                    Json(ErrorResponse::with_code(
                        format!("Failed to send notification: {e}"),
                        "TELEGRAM_API_ERROR".to_string(),
                    )),
                ))
            }
        }
    }
}

/// POST /send - Alias for /notify
pub async fn send(
    state: State<Arc<AppState>>,
    request: JsonExtractor<SendNotificationRequest>,
) -> Result<Json<SendNotificationResponse>, (StatusCode, Json<ErrorResponse>)> {
    notify(state, request).await
}

fn extract_message_id(result: &Option<Value>) -> Option<i64> {
    result.as_ref()?.get("message_id")?.as_i64()
}

// Convenience function for logging
fn info(msg: &str) {
    tracing::info!("{}", msg);
}
