use thiserror::Error;
#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("Fast websocket error: {0}")]
    FastWebSocket(fastwebsockets::WebSocketError),

    #[error("WebSocket error: {0}")]
    WebSocket(tokio_tungstenite::tungstenite::Error),

    #[error("Deserialising JSON error: {error} for payload: {payload}")]
    Deserialize {
        error: serde_json::Error,
        payload: String,
    },

    #[error("No data")]
    NoData,

    #[error("Exchange stream terminated: {0}")]
    Terminated(String),
}
