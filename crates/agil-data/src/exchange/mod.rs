use crate::{
    exchange::subscription::{PrivateExchangeSub, PublicExchangeSub},
    subscriber::{validator::SubscriptionValidator, Subscriber},
    subscription::SubKind,
    ExchangeTransformer, MarketStream,
};
use agil_integration::{
    account::ApiKey, instrument::Instrument, net::url::Url, websocket::fastws::FastWsMessage,
    Validator,
};
use serde::de::DeserializeOwned;

///`Bybit` implementation
pub mod bybit;

///`Paradex` implementation
pub mod paradex;

/// Defines genetic [`ExchangeSub`] ([`PublicExchangeSub`][`PrivateExchangeSub`])
/// used by the exchange [`Connector`] to to build [`FastWsMessage`] subscription payloads.
pub mod subscription;

pub trait Connector
where
    Self: Sized,
{
    /// Unique identifier for the exchange server.
    const ID: ExchangeId;

    /// Type that defines how to translate a [`Subscription`]
    /// into a exchange specifiec channel to be subscribed.
    type Channel: AsRef<str>;

    /// Type that defines how to translate a [`Market`]
    /// into a exchange specific channel to be subscribed.
    type Market: AsRef<str>;

    /// [`Subscriber`] type that subscribes to a [`Subscription`]
    /// over socket
    type Subscriber: Subscriber;

    /// Deserialisable type that [`SubValidator`] expects to receive from
    /// the exchange, it should implement a [`Validator`] that validates the
    /// success of the subscription
    type SubResponse: Validator + std::fmt::Debug + DeserializeOwned;

    /// [`SubscriptionValidator`] that listens to responses from the exchange,
    /// and validates if successful
    type SubValidator: SubscriptionValidator;

    /// A [`ApiKey`] implementations that also implements [`Signer`]
    type Key: ApiKey + Send;

    /// Base [`Url`] of the exchange being connected to.
    fn url() -> Url;

    /// Defines how to translate a collection of [`PublicExchangeSub`]'s into the
    /// [`FastWsMessage`] subscription paylad sent to the server
    fn public_requests(
        exchange_subs: Vec<PublicExchangeSub<Self::Channel, Self::Market>>,
    ) -> Vec<FastWsMessage<'static>>;

    /// Defines how to translate a collection of [`PrivateExchangeSub`]'s into the
    /// [`FastWsMessage`] subscription payload sent to the server
    fn private_requests(
        exchange_subs: Vec<PrivateExchangeSub<Self::Channel>>,
    ) -> Vec<FastWsMessage<'static>>;

    /// Defines how to translate a [`ApiKey`] into the
    /// [`FastWsMessage`] specifiec payload to connect to
    /// the private endpoint
    fn private_connect(api_key: Self::Key) -> FastWsMessage<'static>;

    /// Defines a optional custom [`PingInterval`]
    fn ping_interval() -> Option<PingInterval> {
        None
    }

    /// Expected [`Duration`] the [`SubscriptionValidator`] will wait to receive
    /// the success responses of [`Subscription`]'s sent to the exchange
    fn subscription_timeout() -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }

    /// Number of [`Subscription`] the [`SubscriptionValidator`] will wait for
    fn expected_responses(map: &[Instrument]) -> usize {
        map.len()
    }
}

/// Used when a exchange has different servers, normally [`InstrumentKind`]
/// distinc between servers.
pub trait ExchangeServer: Default + std::fmt::Debug + Clone + Send {
    const ID: ExchangeId;

    /// Base [`Url`] of the exchange being connected to.
    fn websocket_url() -> Url;
}

/// Defines frequency and function for custom pings
pub struct PingInterval {
    pub interval: tokio::time::Interval,
    pub ping: fn() -> FastWsMessage<'static>,
}

/// Unique identifiers for exchange server [`Connector`].
#[derive(Debug, Clone)]
pub enum ExchangeId {
    BybitSpot,
    BybitFutures,
    Paradex,
}

/// Defines the MarketStream associented with an exchange
pub trait StreamSelector<Kind, Key>
where
    Self: Connector,
    Kind: SubKind,
    Key: ApiKey,
{
    type Stream: MarketStream<Self, Kind, Key>;
    type Transformer: ExchangeTransformer<Self, Kind>;
}
