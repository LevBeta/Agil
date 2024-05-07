use super::Connector;
use crate::{
    exchange::{
        paradex::{
            apikey::ParadexApiKey, channel::ParadexChannel, market::ParadexMarket,
            subscription::ParadexResponse, trade::ParadexTrade,
        },
        ExchangeId, StreamSelector,
    },
    subscriber::{validator::FastWebSocketSubValidator, WebSocketSubscriber},
    subscription::trade::PublicTrades,
    transformer::StatelessTransformer,
    ExchangeWsStream,
};
use agil_integration::{
    instrument::Instrument,
    net::url::{Protocol, Url},
    websocket::fastws::FastWsMessage,
};
/// Generic [`ParadexPayload`]
pub mod message;

/// [`Subscription`] response type
pub mod subscription;

/// Defines a trait that translates a [`Subscription`] into a exchange
/// specifiec channel
pub mod channel;

/// PUblic trade type for Paradex
pub mod trade;

/// Defines a trait that translates a [`Subscription`] into a exchange
/// specifiec market
pub mod market;

pub mod apikey;

pub struct Paradex;

impl Connector for Paradex {
    const ID: ExchangeId = ExchangeId::Paradex;
    type Key = ParadexApiKey;
    type Channel = ParadexChannel;
    type Market = ParadexMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = FastWebSocketSubValidator;
    type SubResponse = ParadexResponse;

    fn url() -> Url {
        Url::new(
            "ws.api.prod.paradex.trade",
            "/v1",
            Some(443),
            Some(Protocol::WSS),
        )
    }

    fn public_requests(
        exchange_subs: Vec<super::subscription::PublicExchangeSub<Self::Channel, Self::Market>>,
    ) -> Vec<FastWsMessage<'static>> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| format!("{}.{}", sub.channel.as_ref(), sub.market.as_ref()))
            .collect::<Vec<String>>();

        let messages = stream_names
            .into_iter()
            .map(|stream_name| {
                let params = serde_json::json!({
                    "channel": stream_name
                });

                let s = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "subscribe",
                    "params": params,
                    "id": 1
                });

                FastWsMessage::text(fastwebsockets::Payload::Owned(s.to_string().into_bytes()))
            })
            .collect::<Vec<_>>();

        messages
    }

    fn private_connect(
        _api_key: Self::Key,
    ) -> agil_integration::websocket::fastws::FastWsMessage<'static> {
        todo!()
    }

    fn private_requests(
        _exchange_subs: Vec<super::subscription::PrivateExchangeSub<Self::Channel>>,
    ) -> Vec<agil_integration::websocket::fastws::FastWsMessage<'static>> {
        Vec::new()
    }

    fn ping_interval() -> Option<super::PingInterval> {
        None
    }

    fn expected_responses(map: &[Instrument]) -> usize {
        map.len()
    }
}

impl StreamSelector<PublicTrades, ParadexApiKey> for Paradex {
    type Stream = ExchangeWsStream;
    type Transformer = StatelessTransformer<Self, PublicTrades, ParadexTrade>;
}
