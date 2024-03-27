use crate::websocket::{error::WebSocketError, WebSocket, WsError, WsMessage};
use futures::Stream;
use serde::de::DeserializeOwned;
use tokio_tungstenite::tungstenite::protocol::{frame::Frame, CloseFrame};

/// Process a payload of `String` by deserialising into an `ExchangeMessage`.
pub fn process_text<ExchangeMessage>(
    payload: String,
) -> Option<Result<ExchangeMessage, WebSocketError>>
where
    ExchangeMessage: DeserializeOwned,
{
    Some(
        serde_json::from_str::<ExchangeMessage>(&payload)
            .map_err(|error| WebSocketError::Deserialise { error, payload }),
    )
}

/// Process a payload of `Vec<u8>` bytes by deserialising into an `ExchangeMessage`.
pub fn process_binary<ExchangeMessage>(
    payload: Vec<u8>,
) -> Option<Result<ExchangeMessage, WebSocketError>>
where
    ExchangeMessage: DeserializeOwned,
{
    Some(
        serde_json::from_slice::<ExchangeMessage>(&payload).map_err(|error| {
            WebSocketError::Deserialise {
                error,
                payload: String::from_utf8(payload).unwrap_or_else(|x| x.to_string()),
            }
        }),
    )
}

/// Basic process for a [`WebSocket`] ping message. Logs the payload at `trace` level.
pub fn process_ping<ExchangeMessage>(
    _ping: Vec<u8>,
) -> Option<Result<ExchangeMessage, WebSocketError>> {
    None
}

/// Basic process for a [`WebSocket`] pong message. Logs the payload at `trace` level.
pub fn process_pong<ExchangeMessage>(
    _pong: Vec<u8>,
) -> Option<Result<ExchangeMessage, WebSocketError>> {
    None
}

pub fn process_frame<ExchangeMessage>(
    _frame: Frame,
) -> Option<Result<ExchangeMessage, WebSocketError>> {
    None
}

/// Basic process for a [`WebSocket`] CloseFrame message. Logs the payload at `trace` level.
pub fn process_close_frame<ExchangeMessage>(
    close_frame: Option<CloseFrame<'_>>,
) -> Option<Result<ExchangeMessage, WebSocketError>> {
    let close_frame = format!("{:?}", close_frame);
    Some(Err(WebSocketError::Terminated(close_frame)))
}

pub trait StreamParser {
    type Stream: Stream;
    type Message;
    type Error;

    fn parse<Output>(
        input: Result<Self::Message, Self::Error>,
    ) -> Option<Result<Output, WebSocketError>>
    where
        Output: DeserializeOwned;
}

pub struct WebSocketParser;

impl StreamParser for WebSocketParser {
    type Stream = WebSocket;
    type Message = WsMessage;
    type Error = WsError;

    fn parse<Output>(
        input: Result<Self::Message, Self::Error>,
    ) -> Option<Result<Output, WebSocketError>>
    where
        Output: DeserializeOwned,
    {
        match input {
            Ok(ws_message) => match ws_message {
                WsMessage::Text(text) => process_text(text),
                WsMessage::Binary(binary) => process_binary(binary),
                WsMessage::Ping(ping) => process_ping(ping),
                WsMessage::Pong(pong) => process_pong(pong),
                WsMessage::Close(close_frame) => process_close_frame(close_frame),
                WsMessage::Frame(frame) => process_frame(frame),
            },
            Err(ws_err) => Some(Err(WebSocketError::WebSocket(ws_err))),
        }
    }
}
