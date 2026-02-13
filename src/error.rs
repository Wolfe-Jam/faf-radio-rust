use thiserror::Error;

#[derive(Error, Debug)]
pub enum RadioError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid frequency: {0}. Must be between 40.0 and 108.0 FM")]
    InvalidFrequency(String),

    #[error("Not connected")]
    NotConnected,

    #[error("Already connected")]
    AlreadyConnected,

    #[error("Connection closed: {0}")]
    ConnectionClosed(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Timeout waiting for response")]
    Timeout,

    #[error("Max reconnect attempts reached")]
    MaxReconnectAttemptsReached,
}

pub type Result<T> = std::result::Result<T, RadioError>;
