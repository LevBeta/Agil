/// Support for faster WebSockets using FastWebSockets
/// <https://docs.rs/fastwebsockets/latest/fastwebsockets/index.html>
pub mod fastws;

/// Errors related to [`WebSocket`] and protocol implementatios
pub mod error;

/// Utilies for [`WsMessage`]
pub mod message;
