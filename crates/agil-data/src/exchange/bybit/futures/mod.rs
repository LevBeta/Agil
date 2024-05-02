use super::Bybit;
use crate::exchange::{ExchangeId, ExchangeServer};
use agil_integration::net::url::{Protocol, Url};

/// [`Bybit`] Futures exchange server, convenient type.
pub type BybitFutures = Bybit<BybitFuturesServer>;

/// [`Bybit`] spot [`ExchangeServer`]
#[derive(Default, Debug, Clone)]
pub struct BybitFuturesServer;

impl ExchangeServer for BybitFuturesServer {
    const ID: ExchangeId = ExchangeId::BybitFutures;

    fn websocket_url() -> Url {
        Url {
            domain: String::from("stream.bybit.com"),
            path: String::from("/v5/public/linear"),
            port: Some(443),
            protocol: Some(Protocol::WSS),
        }
    }
}
