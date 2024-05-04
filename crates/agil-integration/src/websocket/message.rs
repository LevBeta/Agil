use crate::websocket::error::WebSocketError;
use serde::de::DeserializeOwned;

/// Process a payload of `String` by deserialising into an `ExchangeMessage`.
pub fn process_text<ExchangeMessage>(
    payload: String,
) -> Option<Result<ExchangeMessage, WebSocketError>>
where
    ExchangeMessage: DeserializeOwned,
{
    Some(
        serde_json::from_str::<ExchangeMessage>(&payload)
            .map_err(|error| WebSocketError::Deserialize { error, payload }),
    )
}

/// Stream parser that takes a Message and deserializes into a output
pub trait StreamParser {
    type Message;
    type Error;

    fn parse<Output>(
        input: Result<Self::Message, Self::Error>,
    ) -> Option<Result<Output, WebSocketError>>
    where
        Output: DeserializeOwned;
}

/// Simple implementation to deserialize [`WsMessage`] into a given Output
pub struct WebSocketParser;
