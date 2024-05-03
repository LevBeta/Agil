use crate::{error::AgilDataError, exchange::Connector, SubKind};
use agil_integration::{
    instrument::Instrument,
    websocket::{
        fastws::{message::FastWebSocketParser, FastWebSocket},
        message::StreamParser,
    },
    Validator,
};
use trait_variant::make;

#[make(Send)]
pub trait SubscriptionValidator {
    type Parser: StreamParser;

    async fn validate<Exchange, Kind>(
        websocket: &mut FastWebSocket,
        map: Vec<Instrument>,
    ) -> Result<(), AgilDataError>
    where
        Exchange: Connector + Send,
        Kind: SubKind + Send;
}

/// Standard [`SubscriptionValidator`] for [`FastWebSocket`]
pub struct FastWebSocketSubValidator;

impl SubscriptionValidator for FastWebSocketSubValidator {
    type Parser = FastWebSocketParser;
    async fn validate<Exchange, Kind>(
        websocket: &mut FastWebSocket,
        map: Vec<Instrument>,
    ) -> Result<(), AgilDataError>
    where
        Exchange: Connector + Send,
        Kind: SubKind + Send,
    {
        // Exchange specific parameters
        let timeout = Exchange::subscription_timeout();
        let expected_responses = Exchange::expected_responses(&map);

        // Keep trapt of current successful subscriptions.
        let mut success_responses = 0usize;

        // TODO
        // Too much indentetion, and errors are typed wrongly
        loop {
            if success_responses == expected_responses {
                break Ok(());
            }

            tokio::select! {
                _ = tokio::time::sleep(timeout) => {
                    break Err(AgilDataError::ValidationTimeout(timeout));
                },
                message = websocket.read_frame() => {
                    match message {
                        Ok(response) => {
                            match Self::Parser::parse::<Exchange::SubResponse>(Ok(response)) {
                                Some(Ok(parsed_response)) => {
                                    match parsed_response.validate() {
                                        Ok(_) => {
                                            success_responses += 1;
                                        },
                                        Err(_) => break Err(AgilDataError::SubscriptionValidation),
                                    }
                                },
                                Some(Err(agil_integration::websocket::error::WebSocketError::Deserialize { .. })) => {
                                    continue;
                                },
                                None => {
                                    break Err(AgilDataError::ParseError);
                                },
                                _ => {
                                    continue
                                }
                            }
                        },
                        Err(_) => {
                            break Err(AgilDataError::WSERROR);
                        },
                    }
                }
            }
        }
    }
}
