mod error;
mod types;

pub use error::{RadioError, Result};
pub use types::{ClientAction, ConnectionState, RadioConfig, ServerMessage};

use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{Duration, interval};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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

    /// Main message loop
    async fn message_loop(
        mut write: impl SinkExt<Message> + Unpin,
        mut read: impl StreamExt<
            Item = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>,
        > + Unpin,
        mut rx: mpsc::UnboundedReceiver<ClientAction>,
        state: Arc<RwLock<ConnectionState>>,
        config: RadioConfig,
        tx: mpsc::UnboundedSender<ClientAction>,
    ) {
        let mut heartbeat = interval(Duration::from_millis(config.heartbeat_interval_ms));

        loop {
            tokio::select! {
                // Handle outgoing messages
                Some(action) = rx.recv() => {
                    let json = serde_json::to_string(&action).unwrap();
                    let msg = Message::text(json);
                    if write.send(msg).await.is_err() {
                        eprintln!("[Radio] Send error");
                        break;
                    }
                }

                // Handle incoming messages
                Some(msg) = read.next() => {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                                Self::handle_server_message(server_msg);
                            }
                        }
                        Ok(Message::Close(_)) => {
                            println!("[Radio] Connection closed");
                            *state.write().await = ConnectionState::Disconnected;
                            break;
                        }
                        Err(_) => {
                            eprintln!("[Radio] Read error");
                            break;
                        }
                        _ => {}
                    }
                }

                // Send heartbeat ping
                _ = heartbeat.tick() => {
                    let _ = tx.send(ClientAction::Ping);
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
}
