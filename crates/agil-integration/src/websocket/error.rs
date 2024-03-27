use thiserror::Error;
#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket error: {0}")]
    WebSocket(tokio_tungstenite::tungstenite::Error),

    #[error("Deserialising JSON error: {error} for payload: {payload}")]
    Deserialise {
        error: serde_json::Error,
        payload: String,
    },

    #[error("Exchange stream terminated: {0}")]
    Terminated(String),
}
