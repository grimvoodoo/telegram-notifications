use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const TELEGRAM_API_BASE: &str = "https://api.telegram.org/bot";

#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    pub chat_id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TelegramResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
}

pub struct TelegramBot {
    client: Client,
    api_url: String,
}

impl TelegramBot {
    pub fn new(bot_token: String) -> Self {
        let api_url = format!("{TELEGRAM_API_BASE}{bot_token}");
        Self {
            client: Client::new(),
            api_url,
        }
    }

    pub async fn send_message(&self, chat_id: &str, message: &str) -> Result<TelegramResponse> {
        self.send_message_advanced(chat_id, message, Some("Markdown"), false)
            .await
    }

    pub async fn send_message_advanced(
        &self,
        chat_id: &str,
        message: &str,
        parse_mode: Option<&str>,
        disable_notification: bool,
    ) -> Result<TelegramResponse> {
        let request = SendMessageRequest {
            chat_id: chat_id.to_string(),
            text: message.to_string(),
            parse_mode: parse_mode.map(|s| s.to_string()),
            disable_notification: if disable_notification {
                Some(true)
            } else {
                None
            },
        };

        let url = format!("{}/sendMessage", self.api_url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Telegram API")?;

        let telegram_response: TelegramResponse = response
            .json()
            .await
            .context("Failed to parse Telegram API response")?;

        if !telegram_response.ok {
            return Err(anyhow::anyhow!(
                "Telegram API error: {} (code: {:?})",
                telegram_response
                    .description
                    .unwrap_or_else(|| "Unknown error".to_string()),
                telegram_response.error_code
            ));
        }

        Ok(telegram_response)
    }

    pub async fn get_me(&self) -> Result<TelegramResponse> {
        let url = format!("{}/getMe", self.api_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send getMe request to Telegram API")?;

        let telegram_response: TelegramResponse = response
            .json()
            .await
            .context("Failed to parse Telegram API response")?;

        if !telegram_response.ok {
            return Err(anyhow::anyhow!(
                "Telegram API error: {} (code: {:?})",
                telegram_response
                    .description
                    .unwrap_or_else(|| "Unknown error".to_string()),
                telegram_response.error_code
            ));
        }

        Ok(telegram_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};
    use serde_json::json;

    // Helper function to create a test bot with mock server URL
    async fn create_test_bot(server: &Server) -> TelegramBot {
        let bot_token = "test_token_123:ABCdefGHIjklMNOpqrSTUvwxyz";
        let mut bot = TelegramBot::new(bot_token.to_string());
        // Override the API URL to use our mock server
        bot.api_url = format!("{}/bot{}", server.url(), bot_token);
        bot
    }

    #[tokio::test]
    async fn test_telegram_bot_new() {
        let bot_token = "123456789:ABCdefGHIjklMNOpqrSTUvwxyz";
        let bot = TelegramBot::new(bot_token.to_string());

        assert!(bot.api_url.contains(bot_token));
        assert!(bot.api_url.starts_with(TELEGRAM_API_BASE));
    }

    #[tokio::test]
    async fn test_send_message_success() {
        let mut server = Server::new_async().await;

        // Mock successful response
        let mock = server
            .mock(
                "POST",
                "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/sendMessage",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": true,
                    "result": {
                        "message_id": 42,
                        "date": 1234567890,
                        "chat": {
                            "id": 987654321,
                            "type": "private"
                        },
                        "text": "Test message"
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot.send_message("987654321", "Test message").await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.ok);
        assert!(response.result.is_some());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_message_advanced_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock(
                "POST",
                "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/sendMessage",
            )
            .match_body(Matcher::JsonString(
                json!({
                    "chat_id": "987654321",
                    "text": "*Bold text*",
                    "parse_mode": "Markdown",
                    "disable_notification": true
                })
                .to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": true,
                    "result": {
                        "message_id": 43,
                        "date": 1234567890,
                        "chat": {
                            "id": 987654321,
                            "type": "private"
                        },
                        "text": "*Bold text*"
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot
            .send_message_advanced("987654321", "*Bold text*", Some("Markdown"), true)
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.ok);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_message_advanced_no_parse_mode() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock(
                "POST",
                "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/sendMessage",
            )
            .match_body(Matcher::JsonString(
                json!({
                    "chat_id": "987654321",
                    "text": "Plain text"
                })
                .to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": true,
                    "result": {
                        "message_id": 44,
                        "date": 1234567890,
                        "chat": {
                            "id": 987654321,
                            "type": "private"
                        },
                        "text": "Plain text"
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot
            .send_message_advanced("987654321", "Plain text", None, false)
            .await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_message_telegram_api_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock(
                "POST",
                "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/sendMessage",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": false,
                    "error_code": 400,
                    "description": "Bad Request: chat not found"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot.send_message("invalid_chat_id", "Test message").await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Bad Request: chat not found"));
        assert!(error.to_string().contains("400"));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_message_network_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock(
                "POST",
                "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/sendMessage",
            )
            .with_status(500)
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot.send_message("987654321", "Test message").await;

        assert!(result.is_err());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_me_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/getMe")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": true,
                    "result": {
                        "id": 123456789,
                        "is_bot": true,
                        "first_name": "Test Bot",
                        "username": "test_bot",
                        "can_join_groups": true,
                        "can_read_all_group_messages": false,
                        "supports_inline_queries": false
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot.get_me().await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.ok);
        assert!(response.result.is_some());

        if let Some(result) = response.result {
            assert_eq!(result["username"], "test_bot");
            assert_eq!(result["first_name"], "Test Bot");
            assert_eq!(result["is_bot"], true);
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_me_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/getMe")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": false,
                    "error_code": 401,
                    "description": "Unauthorized: bot token is invalid"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot.get_me().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Unauthorized: bot token is invalid")
        );
        assert!(error.to_string().contains("401"));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_message_request_serialization() {
        let request = SendMessageRequest {
            chat_id: "123456789".to_string(),
            text: "Hello World".to_string(),
            parse_mode: Some("Markdown".to_string()),
            disable_notification: Some(true),
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["chat_id"], "123456789");
        assert_eq!(parsed["text"], "Hello World");
        assert_eq!(parsed["parse_mode"], "Markdown");
        assert_eq!(parsed["disable_notification"], true);
    }

    #[tokio::test]
    async fn test_send_message_request_serialization_minimal() {
        let request = SendMessageRequest {
            chat_id: "123456789".to_string(),
            text: "Hello World".to_string(),
            parse_mode: None,
            disable_notification: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["chat_id"], "123456789");
        assert_eq!(parsed["text"], "Hello World");
        assert!(parsed.get("parse_mode").is_none());
        assert!(parsed.get("disable_notification").is_none());
    }

    #[tokio::test]
    async fn test_telegram_response_deserialization_success() {
        let json = json!({
            "ok": true,
            "result": {
                "message_id": 42,
                "date": 1234567890,
                "chat": {
                    "id": 987654321,
                    "type": "private"
                },
                "text": "Test message"
            }
        });

        let response: TelegramResponse = serde_json::from_value(json).unwrap();

        assert!(response.ok);
        assert!(response.result.is_some());
        assert!(response.description.is_none());
        assert!(response.error_code.is_none());
    }

    #[tokio::test]
    async fn test_telegram_response_deserialization_error() {
        let json = json!({
            "ok": false,
            "error_code": 400,
            "description": "Bad Request: chat not found"
        });

        let response: TelegramResponse = serde_json::from_value(json).unwrap();

        assert!(!response.ok);
        assert!(response.result.is_none());
        assert_eq!(
            response.description,
            Some("Bad Request: chat not found".to_string())
        );
        assert_eq!(response.error_code, Some(400));
    }

    #[tokio::test]
    async fn test_send_message_calls_send_message_advanced() {
        let mut server = Server::new_async().await;

        // Mock to verify that send_message calls send_message_advanced with correct defaults
        let mock = server
            .mock(
                "POST",
                "/bottest_token_123:ABCdefGHIjklMNOpqrSTUvwxyz/sendMessage",
            )
            .match_body(Matcher::JsonString(
                json!({
                    "chat_id": "987654321",
                    "text": "Test message",
                    "parse_mode": "Markdown"
                })
                .to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": true,
                    "result": {
                        "message_id": 45,
                        "date": 1234567890,
                        "chat": {
                            "id": 987654321,
                            "type": "private"
                        },
                        "text": "Test message"
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let bot = create_test_bot(&server).await;
        let result = bot.send_message("987654321", "Test message").await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[test]
    fn test_telegram_api_base_constant() {
        assert_eq!(TELEGRAM_API_BASE, "https://api.telegram.org/bot");
    }
}
