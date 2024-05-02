use crate::websocket::message::{process_text, StreamParser};
use fastwebsockets::{Frame, OpCode, WebSocketError};

/// Simple implementation to deserialize a FastWebSocket Frame into a given output
pub struct FastWebSocketParser;

impl StreamParser for FastWebSocketParser {
    type Message = Frame<'static>;
    type Error = WebSocketError;

    fn parse<Output>(
        input: Result<Self::Message, Self::Error>,
    ) -> Option<Result<Output, crate::websocket::error::WebSocketError>>
    where
        Output: serde::de::DeserializeOwned,
    {
        match input {
            Ok(ws_message) => match ws_message.opcode {
                OpCode::Text => {
                    process_text(String::from_utf8(ws_message.payload.to_vec()).unwrap())
                }
                _ => todo!(),
            },
            Err(ws_err) => Some(Err(crate::websocket::error::WebSocketError::FastWebSocket(
                ws_err,
            ))),
        }
    }
}
