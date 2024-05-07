use super::Connector;
use crate::{
    exchange::{
        bybit::{
            apikey::BybitApiKey, channel::BybitChannel, market::BybitMarket,
            position::BybitPosition, subscription::BybitResponse, trade::BybitTrade,
        },
        subscription::PublicExchangeSub,
        ExchangeServer, StreamSelector,
    },
    subscriber::{validator::FastWebSocketSubValidator, WebSocketSubscriber},
    subscription::{position::Positions, trade::PublicTrades},
    transformer::StatelessTransformer,
    ExchangeWsStream,
};
use agil_integration::{account::ApiKey, net::url::Url, websocket::fastws::FastWsMessage};

/// [`Bybit`] Spot [`ExchangeServer`]
pub mod spot;

/// [`Bybit`] Futures [`ExchangeServer`]
pub mod futures;

/// Generic [`PublicBybitPayload<T>`] and [`PrivateBybitPayload<T>`]
pub mod message;

/// [`Subscription`] response type
pub mod subscription;

/// Public trade type for Bybit
pub mod trade;

/// Position type for Bybit, private account endpoint
pub mod position;

/// Defines a trait that translates a [`Subscription`] into a exchange
/// specifiec channel
pub mod channel;

/// Defines a trait that translates a [`Subscription`] into a exchange
/// specifiec market
pub mod market;

/// [`ApiKey`] and [`Signer`] implementation for [`Bybit`]
pub mod apikey;

#[derive(Default)]
pub struct Bybit<Server> {
    server: std::marker::PhantomData<Server>,
}

impl<Server: ExchangeServer> Connector for Bybit<Server> {
    const ID: super::ExchangeId = Server::ID;
    type Channel = BybitChannel;
    type Market = BybitMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = FastWebSocketSubValidator;
    type SubResponse = BybitResponse;
    type Key = BybitApiKey;
    fn url() -> Url {
        Server::websocket_url()
    }

    fn public_requests(
        exchange_subs: Vec<PublicExchangeSub<Self::Channel, Self::Market>>,
    ) -> Vec<FastWsMessage<'static>> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| format!("{}.{}", sub.channel.as_ref(), sub.market.as_ref(),))
            .collect::<Vec<String>>();

        vec![FastWsMessage::text(fastwebsockets::Payload::Owned(
            serde_json::json!({
                "op": "subscribe",
                "args": stream_names
            })
            .to_string()
            .into_bytes(),
        ))]
    }

    fn private_requests(
        exchange_subs: Vec<super::subscription::PrivateExchangeSub<Self::Channel>>,
    ) -> Vec<FastWsMessage<'static>> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| format!("{}", sub.channel.as_ref()))
            .collect::<Vec<String>>();

        vec![FastWsMessage::text(fastwebsockets::Payload::Owned(
            serde_json::json!({
                "op": "subscribe",
                "args": stream_names
            })
            .to_string()
            .into_bytes(),
        ))]
    }

    fn private_connect(api_key: Self::Key) -> FastWsMessage<'static> {
        let (sign, expires) = if let Ok(sign_expires) = api_key.clone().sign() {
            sign_expires
        } else {
            todo!();
        };

        //FastWsMessage::text(fastwebsockets::Payload::Owned(
        let x = serde_json::json!({
            "op": "auth",
            "args" : [api_key.api_key, expires, sign]
        })
        .to_string();
        println!("{:?}", x);
        //))
        FastWsMessage::text(fastwebsockets::Payload::Owned(x.into_bytes()))
    }

    fn ping_interval() -> Option<super::PingInterval> {
        Some(super::PingInterval {
            interval: tokio::time::interval(std::time::Duration::from_millis(5_000)),
            ping: || {
                FastWsMessage::text(fastwebsockets::Payload::Owned(
                    serde_json::json!({
                        "op": "ping"
                    })
                    .to_string()
                    .into_bytes(),
                ))
            },
        })
    }

    fn expected_responses(_: &[agil_integration::instrument::Instrument]) -> usize {
        1
    }
}

impl<Server: ExchangeServer + Sync> StreamSelector<PublicTrades, BybitApiKey> for Bybit<Server> {
    type Stream = ExchangeWsStream;
    type Transformer = StatelessTransformer<Self, PublicTrades, BybitTrade>;
}

impl<Server: ExchangeServer + Sync> StreamSelector<Positions, BybitApiKey> for Bybit<Server> {
    type Stream = ExchangeWsStream;
    type Transformer = StatelessTransformer<Self, Positions, BybitPosition>;
}
