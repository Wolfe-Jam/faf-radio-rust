//! WJTTC Tier 2: ENGINE - Core Functionality Tests
//!
//! Tests state management, configuration, serialization,
//! and WebSocket integration for the Radio Protocol client.

use faf_radio_rust::{ClientAction, ConnectionState, RadioClient, RadioConfig, RadioError, ServerMessage};
use serde_json;

// =============================================================================
// STATE MANAGEMENT (Tests 1-6)
// =============================================================================

#[tokio::test]
async fn test_initial_state_disconnected() {
    let client = RadioClient::with_url("wss://example.com");
    assert_eq!(client.state().await, ConnectionState::Disconnected);
}

#[test]
fn test_with_url_constructor() {
    let client = RadioClient::with_url("wss://example.com/radio");
    // with_url should store URL and use defaults
    // We can't access config directly, so verify through behavior
    // The client exists and didn't panic - constructor works
    drop(client);
}

#[test]
fn test_config_default_values() {
    let config = RadioConfig::default();
    assert_eq!(config.heartbeat_interval_ms, 30000);
    assert_eq!(config.max_reconnect_attempts, 5);
    assert_eq!(config.reconnect_delay_ms, 1000);
    assert!(config.auto_reconnect);
    assert_eq!(config.max_reconnect_delay_ms, 16000);
    assert!(config.url.is_empty());
}

#[test]
fn test_config_custom_values() {
    let mut config = RadioConfig::new("wss://custom.server");
    config.heartbeat_interval_ms = 60000;
    config.max_reconnect_attempts = 10;
    config.reconnect_delay_ms = 2000;
    config.auto_reconnect = false;
    config.max_reconnect_delay_ms = 32000;

    assert_eq!(config.url, "wss://custom.server");
    assert_eq!(config.heartbeat_interval_ms, 60000);
    assert_eq!(config.max_reconnect_attempts, 10);
    assert_eq!(config.reconnect_delay_ms, 2000);
    assert!(!config.auto_reconnect);
    assert_eq!(config.max_reconnect_delay_ms, 32000);
}

#[tokio::test]
async fn test_disconnect_when_disconnected() {
    let client = RadioClient::with_url("wss://example.com");
    // Disconnect when already disconnected should be a no-op, returns Ok
    let result = client.disconnect().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_disconnect_sets_state() {
    let client = RadioClient::with_url("wss://example.com");
    client.disconnect().await.unwrap();
    assert_eq!(client.state().await, ConnectionState::Disconnected);
}

// =============================================================================
// TUNE/UNTUNE BEHAVIOR (Tests 7-12)
// =============================================================================

#[tokio::test]
async fn test_connect_already_connected() {
    // Start a local WebSocket server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ws_url = format!("ws://127.0.0.1:{}", addr.port());

    // Accept connections in background
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let _ws = tokio_tungstenite::accept_async(stream).await;
                // Keep connection alive
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            });
        }
    });

    let mut client = RadioClient::with_url(&ws_url);
    let result1 = client.connect().await;
    assert!(result1.is_ok(), "First connect should succeed");

    // Second connect should fail with AlreadyConnected
    let result2 = client.connect().await;
    assert!(result2.is_err());
    assert!(matches!(result2.unwrap_err(), RadioError::AlreadyConnected));
}

#[tokio::test]
async fn test_tune_when_disconnected() {
    let client = RadioClient::with_url("wss://example.com");
    let result = client.tune(vec!["91.0".to_string()]).await;
    // Should fail because not connected (channel rx dropped)
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RadioError::NotConnected));
}

#[tokio::test]
async fn test_untune_when_disconnected() {
    let client = RadioClient::with_url("wss://example.com");
    let result = client.untune(vec!["91.0".to_string()]).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RadioError::NotConnected));
}

#[tokio::test]
async fn test_tune_invalid_freq_before_send() {
    // InvalidFrequency should fire BEFORE NotConnected
    let client = RadioClient::with_url("wss://example.com");
    let result = client.tune(vec!["999.0".to_string()]).await;
    assert!(result.is_err());
    // Validation happens first, so we get InvalidFrequency, not NotConnected
    assert!(matches!(
        result.unwrap_err(),
        RadioError::InvalidFrequency(_)
    ));
}

#[tokio::test]
async fn test_untune_invalid_freq_before_send() {
    let client = RadioClient::with_url("wss://example.com");
    let result = client.untune(vec!["0.0".to_string()]).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        RadioError::InvalidFrequency(_)
    ));
}

#[tokio::test]
async fn test_tune_empty_vec() {
    let client = RadioClient::with_url("wss://example.com");
    let result = client.tune(vec![]).await;
    // Empty vec passes validation but send fails (not connected)
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RadioError::NotConnected));
}

// =============================================================================
// SERIALIZATION (Tests 13-16)
// =============================================================================

#[test]
fn test_tune_action_serialization() {
    let action = ClientAction::Tune {
        frequencies: vec!["91.0".to_string(), "92.5".to_string()],
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["action"], "tune");
    assert_eq!(parsed["frequencies"][0], "91.0");
    assert_eq!(parsed["frequencies"][1], "92.5");
}

#[test]
fn test_untune_action_serialization() {
    let action = ClientAction::Untune {
        frequencies: vec!["94.7".to_string()],
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["action"], "untune");
    assert_eq!(parsed["frequencies"][0], "94.7");
}

#[test]
fn test_ping_action_serialization() {
    let action = ClientAction::Ping;
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["action"], "ping");
}

#[test]
fn test_server_messages_deserialization() {
    // Connected
    let connected_json =
        r#"{"type":"connected","clientId":"abc-123","message":"Welcome","frequencies":{}}"#;
    let msg: ServerMessage = serde_json::from_str(connected_json).unwrap();
    assert!(matches!(msg, ServerMessage::Connected { .. }));

    // Tuned
    let tuned_json = r#"{"type":"tuned","frequencies":["91.0"],"message":"Tuned in"}"#;
    let msg: ServerMessage = serde_json::from_str(tuned_json).unwrap();
    assert!(matches!(msg, ServerMessage::Tuned { .. }));

    // Broadcast
    let broadcast_json = r#"{"type":"broadcast","frequency":"91.0","event":{"data":"hello"},"timestamp":"2026-03-04T00:00:00Z"}"#;
    let msg: ServerMessage = serde_json::from_str(broadcast_json).unwrap();
    assert!(matches!(msg, ServerMessage::Broadcast { .. }));

    // Pong
    let pong_json = r#"{"type":"pong"}"#;
    let msg: ServerMessage = serde_json::from_str(pong_json).unwrap();
    assert!(matches!(msg, ServerMessage::Pong));

    // Error
    let error_json = r#"{"type":"error","message":"Something went wrong"}"#;
    let msg: ServerMessage = serde_json::from_str(error_json).unwrap();
    assert!(matches!(msg, ServerMessage::Error { .. }));
}

// =============================================================================
// LOCAL WEBSOCKET INTEGRATION (Tests 17-18)
// =============================================================================

#[tokio::test]
async fn test_connect_to_local_server() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ws_url = format!("ws://127.0.0.1:{}", addr.port());

    // Accept one connection in background
    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let _ws = tokio_tungstenite::accept_async(stream).await;
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let mut client = RadioClient::with_url(&ws_url);
    assert_eq!(client.state().await, ConnectionState::Disconnected);

    let result = client.connect().await;
    assert!(result.is_ok(), "Should connect to local WS server");
    assert_eq!(client.state().await, ConnectionState::Connected);
}

#[tokio::test]
async fn test_full_lifecycle() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ws_url = format!("ws://127.0.0.1:{}", addr.port());

    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let _ws = tokio_tungstenite::accept_async(stream).await;
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let mut client = RadioClient::with_url(&ws_url);

    // 1. Start disconnected
    assert_eq!(client.state().await, ConnectionState::Disconnected);

    // 2. Connect
    client.connect().await.unwrap();
    assert_eq!(client.state().await, ConnectionState::Connected);

    // 3. Disconnect
    client.disconnect().await.unwrap();
    assert_eq!(client.state().await, ConnectionState::Disconnected);
}
