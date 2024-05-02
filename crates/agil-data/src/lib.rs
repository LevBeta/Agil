use crate::{
    error::AgilDataError,
    event::MarketEvent,
    exchange::Connector,
    subscriber::Subscriber,
    subscription::{SubKind, Subscription},
    transformer::ExchangeTransformer,
};
use agil_integration::account::ApiKey;
use agil_integration::websocket::fastws::FastWebSocket;
use agil_integration::websocket::{message::WebSocketParser, ExchangeStream, WsStream};
use futures::Stream;
use trait_variant::make;
/// Standard implementation to subscribe to a WebSocket
pub mod subscription;

/// Implementation for each exchange
pub mod exchange;

/// Standard implementation for subscribing to a WebSocket
pub mod subscriber;

/// Collections of errors generated by Agil-data
pub mod error;

/// Transformer
pub mod transformer;

pub mod stream;

/// Normalized event's
pub mod event;

pub trait Identifier<T> {
    fn id(&self) -> T;
}

/// A String identifier for a stream that has been subscribed
#[derive(Clone, Debug)]
pub struct SubscriptionId(pub String);

impl<S> From<S> for SubscriptionId
where
    S: Into<String>,
{
    fn from(input: S) -> Self {
        Self(input.into())
    }
}

pub type ExchangeWsStream<Transformer> = ExchangeStream<Transformer, WebSocketParser, WsStream>;

#[make(Send)]
pub trait MarketStream<Exchange, Kind, Key>
where
    Self: Stream<Item = Result<MarketEvent<Kind::Event>, AgilDataError>> + Send + Sized + Unpin,
    Exchange: Connector,
    Kind: SubKind,
    Key: ApiKey,
{
    async fn init(
        subscriptions: &[Subscription<Exchange, Kind>],
        api_key: Option<Key>,
    ) -> Result<FastWebSocket, AgilDataError>
    where
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>;
}

impl<Exchange, Kind, Transformer, Key> MarketStream<Exchange, Kind, Key>
    for ExchangeWsStream<Transformer>
where
    Exchange: Connector<Key = Key> + Send + Sync,
    Kind: SubKind + Send + Sync,
    Transformer: ExchangeTransformer<Exchange, Kind> + Send,
    Kind::Event: Send,
    Key: ApiKey + Send + Sync,
{
    async fn init(
        subscriptions: &[Subscription<Exchange, Kind>],
        api_key: Option<Key>,
    ) -> Result<FastWebSocket, AgilDataError>
    where
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        let mut websocket =
            Exchange::Subscriber::subscribe::<Exchange, Kind, Key>(subscriptions, api_key).await?;

        //let _ = websocket.read_frame().await;

        //let mut transformer = Transformer::new();

        Ok(websocket)
    }
}
