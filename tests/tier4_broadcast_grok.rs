//! WJTTC Tier 4: BROADCAST & GROK — v0.2.0 API Coverage
//!
//! Championship-grade tests for RadioConfig::grok(), ClientAction::Broadcast,
//! and RadioClient::broadcast(). For xAI. Brakes MUST work.

use faf_radio_rust::{ClientAction, ConnectionState, RadioClient, RadioConfig, RadioError, ServerMessage};
use serde_json;

// =============================================================================
// T4.1 GROK PRESET — BRAKES (Tests 1-5)
// =============================================================================

#[test]
fn test_grok_preset_url_exact() {
    let config = RadioConfig::grok();
    assert_eq!(
        config.url,
        "wss://faf-beacon.wolfejam2020.workers.dev/radio",
        "Grok preset must use the exact beacon URL"
    );
}

#[test]
fn test_grok_preset_inherits_defaults() {
    let config = RadioConfig::grok();
    let defaults = RadioConfig::default();

    assert_eq!(config.auto_reconnect, defaults.auto_reconnect);
    assert_eq!(config.max_reconnect_attempts, defaults.max_reconnect_attempts);
    assert_eq!(config.reconnect_delay_ms, defaults.reconnect_delay_ms);
    assert_eq!(config.max_reconnect_delay_ms, defaults.max_reconnect_delay_ms);
    assert_eq!(config.heartbeat_interval_ms, defaults.heartbeat_interval_ms);
}

#[test]
fn test_grok_preset_is_clone_safe() {
    let config = RadioConfig::grok();
    let cloned = config.clone();
    assert_eq!(config.url, cloned.url);
    assert_eq!(config.max_reconnect_attempts, cloned.max_reconnect_attempts);
}

#[test]
fn test_grok_preset_creates_valid_client() {
    let client = RadioClient::new(RadioConfig::grok());
    // Must not panic — client is usable
    assert!(client.validate_frequencies(&["91.0".to_string()]).is_ok());
}

#[tokio::test]
async fn test_grok_client_starts_disconnected() {
    let client = RadioClient::new(RadioConfig::grok());
    assert_eq!(client.state().await, ConnectionState::Disconnected);
}

// =============================================================================
// T4.2 BROADCAST SERIALIZATION — ENGINE (Tests 6-12)
// =============================================================================

#[test]
fn test_broadcast_serializes_action_tag() {
    let action = ClientAction::Broadcast {
        frequency: "91.0".to_string(),
        event: serde_json::json!({}),
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["action"], "broadcast");
}

#[test]
fn test_broadcast_serializes_frequency() {
    let action = ClientAction::Broadcast {
        frequency: "94.7".to_string(),
        event: serde_json::json!({"k": "v"}),
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["frequency"], "94.7");
}

#[test]
fn test_broadcast_serializes_complex_event() {
    let event = serde_json::json!({
        "type": "fafb",
        "project": "faf-cli",
        "yaml_bytes": 4188,
        "fafb_bytes": 220,
        "ratio": "19.0x",
        "sections": 4,
        "nested": {"a": [1, 2, 3]}
    });
    let action = ClientAction::Broadcast {
        frequency: "91.0".to_string(),
        event: event.clone(),
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["event"]["type"], "fafb");
    assert_eq!(parsed["event"]["yaml_bytes"], 4188);
    assert_eq!(parsed["event"]["fafb_bytes"], 220);
    assert_eq!(parsed["event"]["ratio"], "19.0x");
    assert_eq!(parsed["event"]["nested"]["a"][1], 2);
}

#[test]
fn test_broadcast_round_trip_serde() {
    let action = ClientAction::Broadcast {
        frequency: "92.5".to_string(),
        event: serde_json::json!({"soul": "nelly", "version": 3}),
    };
    let json = serde_json::to_string(&action).unwrap();
    let deserialized: ClientAction = serde_json::from_str(&json).unwrap();

    // Verify round-trip fidelity
    match deserialized {
        ClientAction::Broadcast { frequency, event } => {
            assert_eq!(frequency, "92.5");
            assert_eq!(event["soul"], "nelly");
            assert_eq!(event["version"], 3);
        }
        _ => panic!("Expected Broadcast variant after round-trip"),
    }
}

#[test]
fn test_broadcast_with_empty_event() {
    let action = ClientAction::Broadcast {
        frequency: "91.0".to_string(),
        event: serde_json::json!({}),
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["action"], "broadcast");
    assert_eq!(parsed["event"], serde_json::json!({}));
}

#[test]
fn test_broadcast_with_null_event_value() {
    let action = ClientAction::Broadcast {
        frequency: "91.0".to_string(),
        event: serde_json::Value::Null,
    };
    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["action"], "broadcast");
    assert!(parsed["event"].is_null());
}

#[test]
fn test_broadcast_with_array_event() {
    let action = ClientAction::Broadcast {
        frequency: "91.0".to_string(),
        event: serde_json::json!([1, "two", 3.0, null, true]),
    };
    let json = serde_json::to_string(&action).unwrap();
    let deserialized: ClientAction = serde_json::from_str(&json).unwrap();

    match deserialized {
        ClientAction::Broadcast { event, .. } => {
            assert!(event.is_array());
            assert_eq!(event.as_array().unwrap().len(), 5);
        }
        _ => panic!("Expected Broadcast variant"),
    }
}

// =============================================================================
// T4.3 BROADCAST FREQUENCY VALIDATION — BRAKES (Tests 13-18)
// =============================================================================

#[tokio::test]
async fn test_broadcast_rejects_frequency_below_range() {
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("39.9", serde_json::json!({})).await;
    assert!(matches!(result.unwrap_err(), RadioError::InvalidFrequency(_)));
}

#[tokio::test]
async fn test_broadcast_rejects_frequency_above_range() {
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("108.1", serde_json::json!({})).await;
    assert!(matches!(result.unwrap_err(), RadioError::InvalidFrequency(_)));
}

#[tokio::test]
async fn test_broadcast_rejects_non_numeric_frequency() {
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("grok", serde_json::json!({})).await;
    assert!(matches!(result.unwrap_err(), RadioError::InvalidFrequency(_)));
}

#[tokio::test]
async fn test_broadcast_rejects_injection_in_frequency() {
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("91.0; DROP TABLE", serde_json::json!({})).await;
    assert!(matches!(result.unwrap_err(), RadioError::InvalidFrequency(_)));
}

#[tokio::test]
async fn test_broadcast_accepts_boundary_min() {
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("40.0", serde_json::json!({})).await;
    // Validation passes, fails on NotConnected (no live WS)
    assert!(matches!(result.unwrap_err(), RadioError::NotConnected));
}

#[tokio::test]
async fn test_broadcast_accepts_boundary_max() {
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("108.0", serde_json::json!({})).await;
    // Validation passes, fails on NotConnected (no live WS)
    assert!(matches!(result.unwrap_err(), RadioError::NotConnected));
}

// =============================================================================
// T4.4 BROADCAST VALIDATION ORDER — BRAKES (Tests 19-20)
// =============================================================================

#[tokio::test]
async fn test_broadcast_validates_before_send() {
    // InvalidFrequency must fire BEFORE NotConnected
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("0.0", serde_json::json!({"big": "payload"})).await;
    assert!(result.is_err());
    assert!(
        matches!(result.unwrap_err(), RadioError::InvalidFrequency(_)),
        "Validation must happen before channel send"
    );
}

#[tokio::test]
async fn test_broadcast_not_connected_after_validation() {
    // Valid frequency + disconnected = NotConnected (not InvalidFrequency)
    let client = RadioClient::new(RadioConfig::grok());
    let result = client.broadcast("91.0", serde_json::json!({})).await;
    assert!(
        matches!(result.unwrap_err(), RadioError::NotConnected),
        "Valid frequency should pass validation, fail on send"
    );
}

// =============================================================================
// T4.5 BROADCAST OVER LIVE WEBSOCKET — ENGINE (Tests 21-23)
// =============================================================================

#[tokio::test]
async fn test_broadcast_over_live_connection() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ws_url = format!("ws://127.0.0.1:{}", addr.port());

    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let ws = tokio_tungstenite::accept_async(stream).await.unwrap();
            let (_write, mut read) = futures_util::StreamExt::split(ws);
            // Read the broadcast message
            while let Some(msg) = futures_util::StreamExt::next(&mut read).await {
                if msg.is_ok() { break; }
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let mut client = RadioClient::with_url(&ws_url);
    client.connect().await.unwrap();

    let result = client.broadcast("91.0", serde_json::json!({
        "type": "fafb",
        "project": "test",
    })).await;
    assert!(result.is_ok(), "Broadcast should succeed over live WS");

    client.disconnect().await.unwrap();
}

#[tokio::test]
async fn test_broadcast_then_tune_lifecycle() {
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
    client.connect().await.unwrap();

    // Tune first, then broadcast — both should work
    assert!(client.tune(vec!["91.0".into()]).await.is_ok());
    assert!(client.broadcast("91.0", serde_json::json!({"seq": 1})).await.is_ok());
    assert!(client.broadcast("92.5", serde_json::json!({"seq": 2})).await.is_ok());

    client.disconnect().await.unwrap();
    assert_eq!(client.state().await, ConnectionState::Disconnected);
}

#[tokio::test]
async fn test_grok_preset_full_lifecycle() {
    // Use grok preset with a local server to prove the flow works
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ws_url = format!("ws://127.0.0.1:{}", addr.port());

    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let _ws = tokio_tungstenite::accept_async(stream).await;
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    // Override URL for local testing but use grok defaults
    let mut config = RadioConfig::grok();
    config.url = ws_url;
    let mut client = RadioClient::new(config);

    client.connect().await.unwrap();
    assert_eq!(client.state().await, ConnectionState::Connected);

    // Full flow: tune + broadcast + disconnect
    assert!(client.tune(vec!["91.0".into()]).await.is_ok());
    assert!(client.broadcast("91.0", serde_json::json!({
        "type": "fafb",
        "project": "xai-demo",
        "yaml_bytes": 1240,
        "fafb_bytes": 404,
    })).await.is_ok());

    client.disconnect().await.unwrap();
    assert_eq!(client.state().await, ConnectionState::Disconnected);
}

// =============================================================================
// T4.6 SERVER BROADCAST MESSAGE — AERO (Tests 24-27)
// =============================================================================

#[test]
fn test_server_broadcast_deserialization() {
    let json = r#"{"type":"broadcast","frequency":"91.0","event":{"type":"fafb","size":220},"timestamp":"2026-03-06T21:00:00Z"}"#;
    let msg: ServerMessage = serde_json::from_str(json).unwrap();
    match msg {
        ServerMessage::Broadcast { frequency, event, timestamp } => {
            assert_eq!(frequency, "91.0");
            assert_eq!(event["type"], "fafb");
            assert_eq!(event["size"], 220);
            assert_eq!(timestamp, "2026-03-06T21:00:00Z");
        }
        _ => panic!("Expected Broadcast variant"),
    }
}

#[test]
fn test_server_broadcast_with_nested_event() {
    let json = r#"{"type":"broadcast","frequency":"92.5","event":{"layers":{"l1":{"name":"brakes","tests":14}}},"timestamp":"2026-03-06T21:00:00Z"}"#;
    let msg: ServerMessage = serde_json::from_str(json).unwrap();
    match msg {
        ServerMessage::Broadcast { event, .. } => {
            assert_eq!(event["layers"]["l1"]["name"], "brakes");
            assert_eq!(event["layers"]["l1"]["tests"], 14);
        }
        _ => panic!("Expected Broadcast variant"),
    }
}

#[test]
fn test_server_broadcast_missing_event_fails() {
    let json = r#"{"type":"broadcast","frequency":"91.0","timestamp":"2026-03-06T21:00:00Z"}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_err(), "Missing event field must fail");
}

#[test]
fn test_server_broadcast_missing_frequency_fails() {
    let json = r#"{"type":"broadcast","event":{},"timestamp":"2026-03-06T21:00:00Z"}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_err(), "Missing frequency field must fail");
}
