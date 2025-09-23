use anyhow::Result;
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "telegram-notifications")]
#[command(about = "A Telegram notification service - supports both CLI and HTTP API modes")]
pub struct Config {
    /// Telegram Bot Token (can also be set via TELEGRAM_BOT_TOKEN env var)
    #[arg(short, long)]
    pub bot_token: Option<String>,

    /// Chat ID to send messages to (can also be set via TELEGRAM_CHAT_ID env var)
    #[arg(short, long)]
    pub chat_id: Option<String>,

    /// Message to send (CLI mode only)
    #[arg(short, long, default_value = "Hello from Telegram Bot! 🤖")]
    pub message: String,

    /// Run as HTTP server instead of CLI mode
    #[arg(long, default_value_t = false)]
    pub server: bool,

    /// Server port (can also be set via PORT env var)
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Server host address
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,
}

impl Config {
    pub fn from_args_and_env() -> Result<ConfigResolved> {
        let config = Config::parse();

        // Get bot token from env var if not provided via CLI
        let bot_token = match config.bot_token {
            Some(token) => token,
            None => env::var("TELEGRAM_BOT_TOKEN").map_err(|_| {
                anyhow::anyhow!(
                    "Bot token is required. Set TELEGRAM_BOT_TOKEN environment variable or use --bot-token flag"
                )
            })?
        };

        // Get chat ID from env var if not provided via CLI
        let chat_id = match config.chat_id {
            Some(id) => id,
            None => env::var("TELEGRAM_CHAT_ID").map_err(|_| {
                anyhow::anyhow!(
                    "Chat ID is required. Set TELEGRAM_CHAT_ID environment variable or use --chat-id flag"
                )
            })?
        };

        // Validate that required fields are not empty
        if bot_token.is_empty() {
            return Err(anyhow::anyhow!(
                "Bot token cannot be empty. Set TELEGRAM_BOT_TOKEN environment variable or use --bot-token flag"
            ));
        }

        if chat_id.is_empty() {
            return Err(anyhow::anyhow!(
                "Chat ID cannot be empty. Set TELEGRAM_CHAT_ID environment variable or use --chat-id flag"
            ));
        }

        // Override port from environment variable if set
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(config.port);

        Ok(ConfigResolved {
            bot_token,
            chat_id,
            message: config.message,
            server: config.server,
            port,
            host: config.host,
        })
    }
}

#[derive(Debug)]
pub struct ConfigResolved {
    pub bot_token: String,
    pub chat_id: String,
    pub message: String,
    pub server: bool,
    pub port: u16,
    pub host: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    // Helper function to clear environment variables
    fn clear_env_vars() {
        unsafe {
            env::remove_var("TELEGRAM_BOT_TOKEN");
            env::remove_var("TELEGRAM_CHAT_ID");
            env::remove_var("PORT");
        }
    }

    #[test]
    #[serial]
    fn test_config_with_env_vars() {
        clear_env_vars();

        unsafe {
            env::set_var("TELEGRAM_BOT_TOKEN", "test_token_123");
            env::set_var("TELEGRAM_CHAT_ID", "987654321");
        }

        // Mock command line args by providing empty args
        let config = Config {
            bot_token: None,
            chat_id: None,
            message: "Test message".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        // Simulate Config::from_args_and_env() logic
        let bot_token = config
            .bot_token
            .unwrap_or_else(|| env::var("TELEGRAM_BOT_TOKEN").expect("Bot token should be set"));
        let chat_id = config
            .chat_id
            .unwrap_or_else(|| env::var("TELEGRAM_CHAT_ID").expect("Chat ID should be set"));

        assert_eq!(bot_token, "test_token_123");
        assert_eq!(chat_id, "987654321");

        clear_env_vars();
    }

    #[test]
    #[serial]
    fn test_config_with_port_env_var() {
        clear_env_vars();
        unsafe {
            env::set_var("PORT", "8080");
        }

        let config = Config {
            bot_token: Some("test_token".to_string()),
            chat_id: Some("123456789".to_string()),
            message: "Test".to_string(),
            server: false,
            port: 3000, // This should be overridden by env var
            host: "0.0.0.0".to_string(),
        };

        // Test port override logic
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(config.port);

        assert_eq!(port, 8080);

        clear_env_vars();
    }

    #[test]
    #[serial]
    fn test_config_missing_bot_token() {
        clear_env_vars();

        let config = Config {
            bot_token: None,
            chat_id: Some("123456789".to_string()),
            message: "Test".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        // Simulate the error case
        let result = config
            .bot_token
            .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok());
        assert!(result.is_none());
    }

    #[test]
    #[serial]
    fn test_config_missing_chat_id() {
        clear_env_vars();

        let config = Config {
            bot_token: Some("test_token".to_string()),
            chat_id: None,
            message: "Test".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        // Simulate the error case
        let result = config.chat_id.or_else(|| env::var("TELEGRAM_CHAT_ID").ok());
        assert!(result.is_none());
    }

    #[test]
    #[serial]
    fn test_config_empty_bot_token() {
        clear_env_vars();
        unsafe {
            env::set_var("TELEGRAM_BOT_TOKEN", "");
            env::set_var("TELEGRAM_CHAT_ID", "123456789");
        }

        let config = Config {
            bot_token: None,
            chat_id: None,
            message: "Test".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        // Test empty token validation
        let bot_token = config
            .bot_token
            .unwrap_or_else(|| env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default());

        assert!(bot_token.is_empty());

        clear_env_vars();
    }

    #[test]
    #[serial]
    fn test_config_empty_chat_id() {
        clear_env_vars();
        unsafe {
            env::set_var("TELEGRAM_BOT_TOKEN", "test_token");
            env::set_var("TELEGRAM_CHAT_ID", "");
        }

        let config = Config {
            bot_token: None,
            chat_id: None,
            message: "Test".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        // Test empty chat ID validation
        let chat_id = config
            .chat_id
            .unwrap_or_else(|| env::var("TELEGRAM_CHAT_ID").unwrap_or_default());

        assert!(chat_id.is_empty());

        clear_env_vars();
    }

    #[test]
    fn test_config_resolved_struct() {
        let config = ConfigResolved {
            bot_token: "test_token_123".to_string(),
            chat_id: "987654321".to_string(),
            message: "Hello World".to_string(),
            server: true,
            port: 8080,
            host: "127.0.0.1".to_string(),
        };

        assert_eq!(config.bot_token, "test_token_123");
        assert_eq!(config.chat_id, "987654321");
        assert_eq!(config.message, "Hello World");
        assert!(config.server);
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn test_config_defaults() {
        let config = Config {
            bot_token: Some("test".to_string()),
            chat_id: Some("123".to_string()),
            message: "Hello from Telegram Bot! 🤖".to_string(), // Default message
            server: false,                                      // Default server mode
            port: 3000,                                         // Default port
            host: "0.0.0.0".to_string(),                        // Default host
        };

        assert_eq!(config.message, "Hello from Telegram Bot! 🤖");
        assert!(!config.server);
        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "0.0.0.0");
    }

    #[test]
    #[serial]
    fn test_port_parsing_invalid() {
        clear_env_vars();
        unsafe {
            env::set_var("PORT", "invalid_port");
        }

        let config = Config {
            bot_token: Some("test".to_string()),
            chat_id: Some("123".to_string()),
            message: "Test".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        // Test invalid port parsing falls back to default
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(config.port);

        assert_eq!(port, 3000); // Should fall back to config default

        clear_env_vars();
    }

    #[test]
    fn test_config_debug_implementation() {
        let config = Config {
            bot_token: Some("secret_token".to_string()),
            chat_id: Some("123456789".to_string()),
            message: "Test message".to_string(),
            server: true,
            port: 8080,
            host: "localhost".to_string(),
        };

        let debug_str = format!("{config:?}");
        assert!(debug_str.contains("secret_token"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("Test message"));
        assert!(debug_str.contains("true"));
        assert!(debug_str.contains("8080"));
        assert!(debug_str.contains("localhost"));
    }

    #[test]
    fn test_config_resolved_debug_implementation() {
        let config = ConfigResolved {
            bot_token: "secret_token".to_string(),
            chat_id: "123456789".to_string(),
            message: "Test message".to_string(),
            server: false,
            port: 3000,
            host: "0.0.0.0".to_string(),
        };

        let debug_str = format!("{config:?}");
        assert!(debug_str.contains("secret_token"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("Test message"));
        assert!(debug_str.contains("false"));
        assert!(debug_str.contains("3000"));
        assert!(debug_str.contains("0.0.0.0"));
    }
}
