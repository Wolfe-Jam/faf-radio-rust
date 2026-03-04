//! WJTTC Tier 3: AERO - Edge Cases & Polish Tests
//!
//! Tests error display formatting, trait behaviors, deserialization
//! edge cases, and configuration boundary conditions.

use mcpaas::{ConnectionState, RadioConfig, RadioError, ServerMessage};
use serde_json;

// =============================================================================
// ERROR DISPLAY (Tests 1-4)
// =============================================================================

#[test]
fn test_error_display_invalid_frequency() {
    let err = RadioError::InvalidFrequency("999.9".to_string());
    let display = format!("{}", err);
    assert!(display.contains("999.9"), "Error should contain the frequency value");
    assert!(display.contains("40.0") || display.contains("108.0"),
        "Error should mention valid range");
}

#[test]
fn test_error_display_not_connected() {
    let err = RadioError::NotConnected;
    let display = format!("{}", err);
    assert!(!display.is_empty(), "NotConnected display should not be empty");
    assert!(display.to_lowercase().contains("not connected") ||
            display.to_lowercase().contains("not_connected"));
}

#[test]
fn test_error_display_already_connected() {
    let err = RadioError::AlreadyConnected;
    let display = format!("{}", err);
    assert!(!display.is_empty());
    assert!(display.to_lowercase().contains("already connected") ||
            display.to_lowercase().contains("already_connected"));
}

#[test]
fn test_all_error_variants_display() {
    let errors: Vec<RadioError> = vec![
        RadioError::InvalidFrequency("test".to_string()),
        RadioError::NotConnected,
        RadioError::AlreadyConnected,
        RadioError::ConnectionClosed("reason".to_string()),
        RadioError::ServerError("msg".to_string()),
        RadioError::Timeout,
        RadioError::MaxReconnectAttemptsReached,
    ];

    for err in &errors {
        let display = format!("{}", err);
        assert!(!display.is_empty(), "Error {:?} should have non-empty display", err);
    }

    // We skip WebSocket, InvalidUrl, Json errors since they require
    // constructing their inner types, but we verify 7 of 10 variants
    assert_eq!(errors.len(), 7);
}

// =============================================================================
// CONNECTION STATE TRAITS (Tests 5-7)
// =============================================================================

#[test]
fn test_connection_state_equality() {
    // Self-equal
    assert_eq!(ConnectionState::Disconnected, ConnectionState::Disconnected);
    assert_eq!(ConnectionState::Connecting, ConnectionState::Connecting);
    assert_eq!(ConnectionState::Connected, ConnectionState::Connected);
    assert_eq!(ConnectionState::Reconnecting, ConnectionState::Reconnecting);

    // Cross-unequal
    assert_ne!(ConnectionState::Disconnected, ConnectionState::Connected);
    assert_ne!(ConnectionState::Connecting, ConnectionState::Reconnecting);
    assert_ne!(ConnectionState::Connected, ConnectionState::Disconnected);
}

#[test]
fn test_connection_state_clone_copy() {
    let state = ConnectionState::Connected;

    // Clone
    let cloned = state.clone();
    assert_eq!(state, cloned);

    // Copy (implicit - no move)
    let copied = state;
    assert_eq!(state, copied);
    assert_eq!(cloned, copied);
}

#[test]
fn test_connection_state_debug() {
    let states = vec![
        ConnectionState::Disconnected,
        ConnectionState::Connecting,
        ConnectionState::Connected,
        ConnectionState::Reconnecting,
    ];

    for state in &states {
        let debug = format!("{:?}", state);
        assert!(!debug.is_empty(), "Debug output should be non-empty for {:?}", state);
    }
}

// =============================================================================
// DESERIALIZATION EDGE CASES (Tests 8-10)
// =============================================================================

#[test]
fn test_unknown_server_message_type() {
    let json = r#"{"type":"unknown_type","data":"hello"}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_err(), "Unknown message type should fail deserialization");
}

#[test]
fn test_missing_required_fields() {
    // Connected message missing clientId
    let json = r#"{"type":"connected","message":"hi","frequencies":{}}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_err(), "Missing clientId should fail");

    // Tuned message missing frequencies
    let json = r#"{"type":"tuned","message":"ok"}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_err(), "Missing frequencies should fail");

    // Broadcast missing timestamp
    let json = r#"{"type":"broadcast","frequency":"91.0","event":{}}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_err(), "Missing timestamp should fail");
}

#[test]
fn test_extra_fields_ignored() {
    // Pong with extra fields should still deserialize
    let json = r#"{"type":"pong","extra_field":"ignored","another":42}"#;
    let result = serde_json::from_str::<ServerMessage>(json);
    assert!(result.is_ok(), "Extra fields should be ignored by serde");
    assert!(matches!(result.unwrap(), ServerMessage::Pong));
}

// =============================================================================
// CONFIG EDGE CASES (Tests 11-12)
// =============================================================================

#[test]
fn test_config_empty_url() {
    let config = RadioConfig::new("");
    assert_eq!(config.url, "");
    // Should still create valid config with defaults
    assert!(config.auto_reconnect);
    assert_eq!(config.max_reconnect_attempts, 5);
}

#[test]
fn test_config_clone() {
    let config = RadioConfig::new("wss://example.com");
    let cloned = config.clone();

    // Independent copy - same values
    assert_eq!(config.url, cloned.url);
    assert_eq!(config.heartbeat_interval_ms, cloned.heartbeat_interval_ms);
    assert_eq!(config.max_reconnect_attempts, cloned.max_reconnect_attempts);
    assert_eq!(config.reconnect_delay_ms, cloned.reconnect_delay_ms);
    assert_eq!(config.auto_reconnect, cloned.auto_reconnect);
}
