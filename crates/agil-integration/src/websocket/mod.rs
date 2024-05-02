use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;

/// Support for faster WebSockets using FastWebSockets
/// <https://docs.rs/fastwebsockets/latest/fastwebsockets/index.html>
pub mod fastws;

/// Errors related to [`WebSocket`] and protocol implementatios
pub mod error;

/// Utilies for [`WsMessage`]
pub mod message;

/// Convenient type alias for a tungstenite `WebSocketStream`.
pub type WebSocket = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Convenient type alias for the `Sink` half of a tungstenite [`WebSocket`].
pub type WsSink = futures::stream::SplitSink<WebSocket, WsMessage>;

/// Convenient type alias for the `Stream` half of a tungstenite [`WebSocket`].
pub type WsStream = futures::stream::SplitStream<WebSocket>;

/// Communicative type alias for a tungstenite [`WebSocket`] `Message`.
pub type WsMessage = tokio_tungstenite::tungstenite::Message;

/// Communicative type alias for a tungstenite [`WebSocket`] `Error`.
pub type WsError = tokio_tungstenite::tungstenite::Error;
