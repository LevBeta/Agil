use agil_integration::websocket::error::WebSocketError;
use thiserror::Error;

/// Errors generated by Agil-Data
#[derive(Debug, Error)]
pub enum AgilDataError {
    #[error("Websocket error: {0}")]
    WebSocketError(#[from] WebSocketError),
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Websocket error")]
    WSERROR,
    #[error("Couldn't subscribe, validation timeout reached")]
    ValidationTimeout(std::time::Duration),
    // This error should be changed, to support responding
    // with the specifiec subscription that failed
    #[error("Failed subscription response")]
    FailedSubscription,
    #[error("Message out of sequence")]
    OutOfSequenceMessage,
    #[error("Error validating a subscription")]
    SubscriptionValidation,
    #[error("Error parsing data")]
    ParseError,
}
