use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    /// Message to send
    pub message: String,

    /// Optional custom chat ID (overrides default)
    pub chat_id: Option<String>,

    /// Optional parse mode (Markdown, HTML, or None)
    pub parse_mode: Option<String>,

    /// Optional disable notification (silent message)
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SendNotificationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_message_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub bot_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ErrorResponse {
    pub fn with_code(error: String, code: String) -> Self {
        Self {
            success: false,
            error,
            code: Some(code),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub description: String,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub description: String,
}

impl Default for InfoResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoResponse {
    pub fn new() -> Self {
        Self {
            name: "Telegram Notifications API".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Send notifications via Telegram Bot API".to_string(),
            endpoints: vec![
                EndpointInfo {
                    method: "GET".to_string(),
                    path: "/".to_string(),
                    description: "API information and available endpoints".to_string(),
                },
                EndpointInfo {
                    method: "GET".to_string(),
                    path: "/health".to_string(),
                    description: "Health check and bot status".to_string(),
                },
                EndpointInfo {
                    method: "POST".to_string(),
                    path: "/notify".to_string(),
                    description: "Send a notification message".to_string(),
                },
                EndpointInfo {
                    method: "POST".to_string(),
                    path: "/send".to_string(),
                    description: "Send a notification message (alias for /notify)".to_string(),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_send_notification_request_deserialization_minimal() {
        let json = r#"{"message": "Test message"}"#;
        let request: SendNotificationRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.message, "Test message");
        assert_eq!(request.chat_id, None);
        assert_eq!(request.parse_mode, None);
        assert_eq!(request.disable_notification, None);
    }

    #[test]
    fn test_send_notification_request_deserialization_full() {
        let json = r#"{
            "message": "Test message",
            "chat_id": "123456789",
            "parse_mode": "Markdown",
            "disable_notification": true
        }"#;
        let request: SendNotificationRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.message, "Test message");
        assert_eq!(request.chat_id, Some("123456789".to_string()));
        assert_eq!(request.parse_mode, Some("Markdown".to_string()));
        assert_eq!(request.disable_notification, Some(true));
    }

    #[test]
    fn test_send_notification_request_empty_message_fails() {
        let json = r#"{}"#;
        let result: Result<SendNotificationRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_send_notification_response_serialization() {
        let response = SendNotificationResponse {
            success: true,
            message: "Notification sent successfully".to_string(),
            telegram_message_id: Some(42),
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["success"], true);
        assert_eq!(parsed["message"], "Notification sent successfully");
        assert_eq!(parsed["telegram_message_id"], 42);
    }

    #[test]
    fn test_send_notification_response_serialization_without_message_id() {
        let response = SendNotificationResponse {
            success: true,
            message: "Notification sent successfully".to_string(),
            telegram_message_id: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["success"], true);
        assert_eq!(parsed["message"], "Notification sent successfully");
        assert!(parsed.get("telegram_message_id").is_none());
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            service: "telegram-notifications".to_string(),
            version: "0.1.0".to_string(),
            bot_verified: true,
            bot_username: Some("test_bot".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["status"], "healthy");
        assert_eq!(parsed["service"], "telegram-notifications");
        assert_eq!(parsed["version"], "0.1.0");
        assert_eq!(parsed["bot_verified"], true);
        assert_eq!(parsed["bot_username"], "test_bot");
    }

    #[test]
    fn test_health_response_serialization_without_username() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            service: "telegram-notifications".to_string(),
            version: "0.1.0".to_string(),
            bot_verified: true,
            bot_username: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["bot_verified"], true);
        assert!(parsed.get("bot_username").is_none());
    }

    #[test]
    fn test_error_response_with_code() {
        let error = ErrorResponse::with_code("Test error".to_string(), "TEST_ERROR".to_string());

        assert_eq!(error.success, false);
        assert_eq!(error.error, "Test error");
        assert_eq!(error.code, Some("TEST_ERROR".to_string()));
    }

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse::with_code("Test error".to_string(), "TEST_ERROR".to_string());

        let json = serde_json::to_string(&error).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["success"], false);
        assert_eq!(parsed["error"], "Test error");
        assert_eq!(parsed["code"], "TEST_ERROR");
    }

    #[test]
    fn test_error_response_serialization_without_code() {
        let error = ErrorResponse {
            success: false,
            error: "Test error".to_string(),
            code: None,
        };

        let json = serde_json::to_string(&error).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["success"], false);
        assert_eq!(parsed["error"], "Test error");
        assert!(parsed.get("code").is_none());
    }

    #[test]
    fn test_info_response_creation() {
        let info = InfoResponse::new();

        assert_eq!(info.name, "Telegram Notifications API");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.description, "Send notifications via Telegram Bot API");
        assert_eq!(info.endpoints.len(), 4);

        // Check specific endpoints
        let root_endpoint = &info.endpoints[0];
        assert_eq!(root_endpoint.method, "GET");
        assert_eq!(root_endpoint.path, "/");

        let health_endpoint = &info.endpoints[1];
        assert_eq!(health_endpoint.method, "GET");
        assert_eq!(health_endpoint.path, "/health");

        let notify_endpoint = &info.endpoints[2];
        assert_eq!(notify_endpoint.method, "POST");
        assert_eq!(notify_endpoint.path, "/notify");

        let send_endpoint = &info.endpoints[3];
        assert_eq!(send_endpoint.method, "POST");
        assert_eq!(send_endpoint.path, "/send");
    }

    #[test]
    fn test_info_response_serialization() {
        let info = InfoResponse::new();

        let json = serde_json::to_string(&info).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["name"], "Telegram Notifications API");
        assert_eq!(parsed["version"], env!("CARGO_PKG_VERSION"));
        assert!(parsed["endpoints"].is_array());
        assert_eq!(parsed["endpoints"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn test_endpoint_info_serialization() {
        let endpoint = EndpointInfo {
            method: "GET".to_string(),
            path: "/test".to_string(),
            description: "Test endpoint".to_string(),
        };

        let json = serde_json::to_string(&endpoint).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["method"], "GET");
        assert_eq!(parsed["path"], "/test");
        assert_eq!(parsed["description"], "Test endpoint");
    }
}
