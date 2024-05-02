use super::Bybit;
use crate::exchange::{ExchangeId, ExchangeServer};
use agil_integration::net::url::{Protocol, Url};

/// [`Bybit`] Spot exchange server, convenient type.
pub type BybitSpot = Bybit<BybitSpotServer>;

/// [`Bybit`] spot [`ExchangeServer`]
#[derive(Default, Debug, Clone)]
pub struct BybitSpotServer;

impl ExchangeServer for BybitSpotServer {
    const ID: ExchangeId = ExchangeId::BybitSpot;

    fn websocket_url() -> Url {
        Url {
            domain: String::from("stream.bybit.com"),
            path: String::from("/v5/public/spot"),
            port: Some(443),
            protocol: Some(Protocol::WSS),
        }
    }
}
