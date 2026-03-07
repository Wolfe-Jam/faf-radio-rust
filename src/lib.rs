mod error;
mod types;

pub use error::{RadioError, Result};
pub use types::{ClientAction, ConnectionState, RadioConfig, ServerMessage};

use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{Duration, interval};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::protocol::Message,
};

type WsWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Radio Protocol WebSocket client
pub struct RadioClient {
    config: RadioConfig,
    state: Arc<RwLock<ConnectionState>>,
    tx: mpsc::UnboundedSender<ClientAction>,
}

impl RadioClient {
    /// Create a new Radio client
    pub fn new(config: RadioConfig) -> Self {
        let (tx, _rx) = mpsc::unbounded_channel();
        Self {
            config,
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            tx,
        }
    }

    /// Create a new Radio client with URL
    pub fn with_url(url: impl Into<String>) -> Self {
        Self::new(RadioConfig::new(url))
    }

    /// Get current connection state
    pub async fn state(&self) -> ConnectionState {
        *self.state.read().await
    }

    /// Connect to the Radio Protocol server
    pub async fn connect(&mut self) -> Result<()> {
        let state = *self.state.read().await;
        if state != ConnectionState::Disconnected {
            return Err(RadioError::AlreadyConnected);
        }

        *self.state.write().await = ConnectionState::Connecting;

        let (ws_stream, _) = connect_async(&self.config.url).await?;
        let (write, read) = ws_stream.split();

        let (tx, rx) = mpsc::unbounded_channel();
        self.tx = tx.clone();

        let state_clone = Arc::clone(&self.state);
        let config_clone = self.config.clone();

        // Spawn message handler task
        tokio::spawn(Self::message_loop(
            write,
            read,
            rx,
            state_clone,
            config_clone,
            tx.clone(),
        ));

        *self.state.write().await = ConnectionState::Connected;
        Ok(())
    }

    /// Disconnect from the server
    pub async fn disconnect(&self) -> Result<()> {
        *self.state.write().await = ConnectionState::Disconnected;
        Ok(())
    }

    /// Tune to frequencies
    pub async fn tune(&self, frequencies: Vec<String>) -> Result<()> {
        self.validate_frequencies(&frequencies)?;
        self.tx
            .send(ClientAction::Tune { frequencies })
            .map_err(|_| RadioError::NotConnected)?;
        Ok(())
    }

    /// Untune from frequencies
    pub async fn untune(&self, frequencies: Vec<String>) -> Result<()> {
        self.validate_frequencies(&frequencies)?;
        self.tx
            .send(ClientAction::Untune { frequencies })
            .map_err(|_| RadioError::NotConnected)?;
        Ok(())
    }

    /// Broadcast an event on a frequency
    pub async fn broadcast(&self, frequency: &str, event: serde_json::Value) -> Result<()> {
        self.validate_frequencies(&[frequency.to_string()])?;
        self.tx
            .send(ClientAction::Broadcast {
                frequency: frequency.to_string(),
                event,
            })
            .map_err(|_| RadioError::NotConnected)?;
        Ok(())
    }

    /// Validate frequency range (40.0-108.0 FM)
    pub fn validate_frequencies(&self, frequencies: &[String]) -> Result<()> {
        for freq in frequencies {
            if let Ok(f) = freq.parse::<f64>() {
                if !(40.0..=108.0).contains(&f) {
                    return Err(RadioError::InvalidFrequency(freq.clone()));
                }
            } else {
                return Err(RadioError::InvalidFrequency(freq.clone()));
            }
        }
        Ok(())
    }

    /// Main message loop with auto-reconnect
    async fn message_loop(
        mut write: WsWrite,
        mut read: WsRead,
        mut rx: mpsc::UnboundedReceiver<ClientAction>,
        state: Arc<RwLock<ConnectionState>>,
        config: RadioConfig,
        tx: mpsc::UnboundedSender<ClientAction>,
    ) {
        let mut heartbeat = interval(Duration::from_millis(config.heartbeat_interval_ms));
        let mut attempt: u32 = 0;

        loop {
            // Inner select loop — runs until connection breaks
            let broke_cleanly = loop {
                tokio::select! {
                    Some(action) = rx.recv() => {
                        let json = serde_json::to_string(&action).unwrap();
                        let msg = Message::text(json);
                        if write.send(msg).await.is_err() {
                            eprintln!("[Radio] Send error");
                            break false;
                        }
                    }

                    Some(msg) = read.next() => {
                        match msg {
                            Ok(Message::Text(text)) => {
                                // Connection is alive — reset attempt counter
                                attempt = 0;
                                if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                                    Self::handle_server_message(server_msg);
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("[Radio] Connection closed by server");
                                break true;
                            }
                            Err(_) => {
                                eprintln!("[Radio] Read error");
                                break false;
                            }
                            _ => {}
                        }
                    }

                    _ = heartbeat.tick() => {
                        let _ = tx.send(ClientAction::Ping);
                    }
                }
            };

            // Should we reconnect?
            if !config.auto_reconnect {
                break;
            }
            if broke_cleanly {
                // Server sent Close frame — intentional disconnect, don't retry
                break;
            }
            if config.max_reconnect_attempts > 0 && attempt >= config.max_reconnect_attempts {
                eprintln!(
                    "[Radio] Max reconnect attempts reached ({})",
                    config.max_reconnect_attempts
                );
                break;
            }

            // Exponential backoff
            attempt += 1;
            let delay = std::cmp::min(
                config.reconnect_delay_ms * 2u64.saturating_pow(attempt - 1),
                config.max_reconnect_delay_ms,
            );
            eprintln!(
                "[Radio] Reconnecting in {}ms (attempt {}/{})...",
                delay,
                attempt,
                if config.max_reconnect_attempts == 0 {
                    "∞".to_string()
                } else {
                    config.max_reconnect_attempts.to_string()
                }
            );

            *state.write().await = ConnectionState::Reconnecting;
            tokio::time::sleep(Duration::from_millis(delay)).await;

            // Attempt reconnection
            match connect_async(&config.url).await {
                Ok((ws_stream, _)) => {
                    let (new_write, new_read) = ws_stream.split();
                    write = new_write;
                    read = new_read;
                    *state.write().await = ConnectionState::Connected;
                    heartbeat = interval(Duration::from_millis(config.heartbeat_interval_ms));
                    eprintln!("[Radio] Reconnected successfully");
                }
                Err(e) => {
                    eprintln!("[Radio] Reconnect failed: {}", e);
                    continue;
                }
            }
        }

        *state.write().await = ConnectionState::Disconnected;
    }

    /// Handle server messages
    fn handle_server_message(msg: ServerMessage) {
        match msg {
            ServerMessage::Connected {
                client_id, message, ..
            } => {
                println!("[Radio] ✅ {} (Client ID: {})", message, client_id);
            }
            ServerMessage::Tuned {
                frequencies,
                message,
            } => {
                println!("[Radio] ✅ {} - {:?}", message, frequencies);
            }
            ServerMessage::Broadcast {
                frequency,
                event,
                timestamp,
            } => {
                println!(
                    "[Radio] 📻 Broadcast on {} FM at {}: {:?}",
                    frequency, timestamp, event
                );
            }
            ServerMessage::Pong => {
                println!("[Radio] 💓 Heartbeat");
            }
            ServerMessage::Error { message } => {
                eprintln!("[Radio] ❌ Error: {}", message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_validation() {
        let config = RadioConfig::new("wss://example.com");
        let client = RadioClient::new(config);

        // Valid frequencies
        assert!(client.validate_frequencies(&["91.0".to_string()]).is_ok());
        assert!(
            client
                .validate_frequencies(&["40.0".to_string(), "108.0".to_string()])
                .is_ok()
        );

        // Invalid frequencies
        assert!(client.validate_frequencies(&["39.9".to_string()]).is_err());
        assert!(client.validate_frequencies(&["108.1".to_string()]).is_err());
        assert!(
            client
                .validate_frequencies(&["invalid".to_string()])
                .is_err()
        );
    }

    #[test]
    fn test_initial_state() {
        let client = RadioClient::with_url("wss://example.com");
        // State check requires async, so we can't test it in a sync test
        // This is just a placeholder for the sync test
        assert_eq!(client.config.url, "wss://example.com");
    }

    #[test]
    fn test_grok_preset() {
        let config = RadioConfig::grok();
        assert_eq!(
            config.url,
            "wss://faf-beacon.wolfejam2020.workers.dev/radio"
        );
        assert!(config.auto_reconnect);
        assert_eq!(config.max_reconnect_attempts, 5);
    }

    #[tokio::test]
    async fn test_broadcast_invalid_frequency() {
        let client = RadioClient::new(RadioConfig::grok());
        let result = client
            .broadcast("999.0", serde_json::json!({"type": "test"}))
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            RadioError::InvalidFrequency(_)
        ));
    }

    #[tokio::test]
    async fn test_broadcast_when_disconnected() {
        let client = RadioClient::new(RadioConfig::grok());
        let result = client
            .broadcast("91.0", serde_json::json!({"type": "test"}))
            .await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RadioError::NotConnected));
    }
}
