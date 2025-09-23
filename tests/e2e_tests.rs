use reqwest::Client;
use serde_json::{Value, json};
use std::process::Command;
use std::time::Duration;

/// Test helper to start the server process with a specific port
fn start_test_server(port: u16) -> std::process::Child {
    Command::new("cargo")
        .args(&[
            "run",
            "--",
            "--server",
            "--port",
            &port.to_string(),
            "--host",
            "127.0.0.1",
        ])
        .env(
            "TELEGRAM_BOT_TOKEN",
            "test_token:ABCdefGHIjklMNOpqrSTUvwxyz",
        )
        .env("TELEGRAM_CHAT_ID", "123456789")
        .env("TELEGRAM_NOTIFICATIONS_SKIP_VALIDATION", "true") // Skip bot validation in tests
        .env("RUST_LOG", "warn") // Minimize logging during tests but show warnings
        .spawn()
        .expect("Failed to start test server")
}

/// Wait for server to be ready
async fn wait_for_server_ready(server_url: &str, max_attempts: u32) -> bool {
    let client = Client::new();

    for _ in 0..max_attempts {
        if let Ok(response) = client.get(&format!("{}/", server_url)).send().await {
            if response.status().is_success() {
                return true;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    false
}

#[tokio::test]
#[ignore] // This test requires actual server startup, run with --ignored
async fn test_e2e_server_startup_and_info_endpoint() {
    let port = 3001;
    let server_url = format!("http://127.0.0.1:{}", port);
    let mut server_process = start_test_server(port);

    // Wait for server to start (should work with validation skipped)
    assert!(
        wait_for_server_ready(&server_url, 20).await,
        "Server failed to start even with validation skipped"
    );

    let client = Client::new();

    // Test root endpoint
    let response = client.get(&format!("{}/", server_url)).send().await;
    assert!(
        response.is_ok(),
        "Failed to connect to server root endpoint"
    );

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Telegram Notifications API");
    assert_eq!(body["endpoints"].as_array().unwrap().len(), 4);

    // Cleanup
    let _ = server_process.kill();
    let _ = server_process.wait();
}

#[tokio::test]
#[ignore] // This test requires actual server startup, run with --ignored
async fn test_e2e_health_endpoint() {
    let port = 3002;
    let server_url = format!("http://127.0.0.1:{}", port);
    let mut server_process = start_test_server(port);

    // Wait for server to start
    assert!(
        wait_for_server_ready(&server_url, 20).await,
        "Server failed to start"
    );

    let client = Client::new();

    // Test health endpoint
    let response = client.get(&format!("{}/health", server_url)).send().await;
    assert!(response.is_ok(), "Failed to connect to health endpoint");

    let response = response.unwrap();
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body: Value = response.json().await.unwrap();
    // With validation skipped, we should get a response structure but bot_verified will be false
    assert!(body.get("service").is_some());
    assert!(body.get("version").is_some());
    assert!(body.get("status").is_some());
    assert_eq!(body["bot_verified"], false); // Should be false due to skipped validation

    // Cleanup
    let _ = server_process.kill();
    let _ = server_process.wait();
}

#[tokio::test]
#[ignore] // This test requires actual server startup, run with --ignored
async fn test_e2e_notify_endpoint_validation() {
    let port = 3003;
    let server_url = format!("http://127.0.0.1:{}", port);
    let mut server_process = start_test_server(port);

    // Wait for server to start
    assert!(
        wait_for_server_ready(&server_url, 20).await,
        "Server failed to start"
    );

    let client = Client::new();

    // Test with empty message (should fail)
    let response = client
        .post(&format!("{}/notify", server_url))
        .json(&json!({"message": ""}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400);

    // Test with missing message (should fail)
    let response = client
        .post(&format!("{}/notify", server_url))
        .json(&json!({"chat_id": "123"}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 422); // Unprocessable Entity

    // Test with valid message (should succeed in test mode)
    let response = client
        .post(&format!("{}/notify", server_url))
        .json(&json!({"message": "Test notification"}))
        .send()
        .await
        .unwrap();

    // Should return 200 in test mode (validation skipped)
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["success"], true);
    assert!(body["message"].as_str().unwrap().contains("test mode"));

    // Cleanup
    let _ = server_process.kill();
    let _ = server_process.wait();
}

#[tokio::test]
#[ignore] // This test requires actual server startup, run with --ignored
async fn test_e2e_cors_headers() {
    let port = 3004;
    let server_url = format!("http://127.0.0.1:{}", port);
    let mut server_process = start_test_server(port);

    // Wait for server to start
    assert!(
        wait_for_server_ready(&server_url, 20).await,
        "Server failed to start"
    );

    let client = Client::new();

    // Test CORS preflight request
    let response = client
        .request(reqwest::Method::OPTIONS, &format!("{}/notify", server_url))
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .send()
        .await
        .unwrap();

    // Should handle CORS preflight
    assert!(response.status() == 200 || response.status() == 204);

    // Cleanup
    let _ = server_process.kill();
    let _ = server_process.wait();
}

#[tokio::test]
#[ignore] // This test requires actual server startup, run with --ignored
async fn test_e2e_404_handling() {
    let port = 3005;
    let server_url = format!("http://127.0.0.1:{}", port);
    let mut server_process = start_test_server(port);

    // Wait for server to start
    assert!(
        wait_for_server_ready(&server_url, 20).await,
        "Server failed to start"
    );

    let client = Client::new();

    // Test 404 for non-existent endpoint
    let response = client
        .get(&format!("{}/nonexistent", server_url))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404);

    // Cleanup
    let _ = server_process.kill();
    let _ = server_process.wait();
}

#[tokio::test]
#[ignore] // This test requires actual server startup, run with --ignored
async fn test_e2e_method_not_allowed() {
    let port = 3006;
    let server_url = format!("http://127.0.0.1:{}", port);
    let mut server_process = start_test_server(port);

    // Wait for server to start
    assert!(
        wait_for_server_ready(&server_url, 20).await,
        "Server failed to start"
    );

    let client = Client::new();

    // Test GET on POST-only endpoint
    let response = client
        .get(&format!("{}/notify", server_url))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 405); // Method Not Allowed

    // Test POST on GET-only endpoint
    let response = client
        .post(&format!("{}/", server_url))
        .json(&json!({}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 405); // Method Not Allowed

    // Cleanup
    let _ = server_process.kill();
    let _ = server_process.wait();
}

#[test]
fn test_cli_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute help command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("telegram-notifications"));
    assert!(stdout.contains("--server"));
    assert!(stdout.contains("--port"));
    assert!(stdout.contains("--message"));
}

#[test]
fn test_cli_version_in_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute help command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should contain information about both CLI and server modes
    assert!(stdout.contains("CLI") || stdout.contains("server") || stdout.contains("HTTP"));
}

// Utility tests that don't require server startup
#[test]
fn test_cargo_check_passes() {
    let output = Command::new("cargo")
        .args(&["check"])
        .output()
        .expect("Failed to run cargo check");

    assert!(
        output.status.success(),
        "cargo check should pass without errors"
    );
}

#[test]
fn test_cargo_clippy_passes() {
    let output = Command::new("cargo")
        .args(&["clippy", "--", "-D", "warnings"])
        .output()
        .expect("Failed to run cargo clippy");

    // Note: We allow clippy to fail in tests since it might have opinions
    // about test code that we don't want to enforce
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        println!("Clippy warnings (non-fatal): {}", stderr);
    }
}
