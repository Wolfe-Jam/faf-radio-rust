use serde::{Deserialize, Serialize};

/// Client connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

/// Client action sent to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ClientAction {
    /// Tune to frequencies
    Tune { frequencies: Vec<String> },
    /// Untune from frequencies
    Untune { frequencies: Vec<String> },
    /// Ping for heartbeat
    Ping,
}

/// Server message received from server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerMessage {
    /// Connection established
    Connected {
        #[serde(rename = "clientId")]
        client_id: String,
        message: String,
        frequencies: serde_json::Value,
    },
    /// Successfully tuned to frequencies
    Tuned {
        frequencies: Vec<String>,
        message: String,
    },
    /// Broadcast received
    Broadcast {
        frequency: String,
        event: serde_json::Value,
        timestamp: String,
    },
    /// Heartbeat response
    Pong,
    /// Error message
    Error { message: String },
}

/// Radio Protocol client configuration
#[derive(Debug, Clone)]
pub struct RadioConfig {
    /// WebSocket server URL
    pub url: String,
    /// Auto-reconnect on disconnect
    pub auto_reconnect: bool,
    /// Maximum reconnection attempts (0 = unlimited)
    pub max_reconnect_attempts: u32,
    /// Initial reconnection delay (milliseconds)
    pub reconnect_delay_ms: u64,
    /// Maximum reconnection delay (milliseconds)
    pub max_reconnect_delay_ms: u64,
    /// Heartbeat interval (milliseconds)
    pub heartbeat_interval_ms: u64,
}

impl Default for RadioConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,      // 1 second
            max_reconnect_delay_ms: 16000, // 16 seconds
            heartbeat_interval_ms: 30000,  // 30 seconds
        }
    }
}

impl RadioConfig {
    /// Create new config with URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }
}
