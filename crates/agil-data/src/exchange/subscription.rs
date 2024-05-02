use crate::{
    subscription::{SubKind, Subscription},
    Identifier, SubscriptionId,
};

/// A dummy trait to group both [`PublicExchangeSub`] and [`PrivateExchangeSub`]
pub(crate) trait ExchangeSub {}

/// Defines an exchange specific market and channel combination used by an exchange
/// normally used for public endpoint that requires both
/// ### Example
/// #### Bybit
/// ```json
/// PublicExchangeSub {
///     channel: BybitChannel("publicTrade"),
///     market: BinanceMarket("btcusdt"),
/// }
/// ```
pub struct PublicExchangeSub<Channel, Market> {
    pub channel: Channel,
    pub market: Market,
}

impl<Channel, Market> PublicExchangeSub<Channel, Market>
where
    Channel: AsRef<str>,
    Market: AsRef<str>,
{
    /// Construct a new exchange specific [`Self`] with [`Subscription`] provided
    pub fn new<Exchange, Kind>(sub: &Subscription<Exchange, Kind>) -> Self
    where
        Subscription<Exchange, Kind>: Identifier<Channel> + Identifier<Market>,
        Kind: SubKind,
    {
        Self {
            channel: sub.id(),
            market: sub.id(),
        }
    }
}

impl<Channel, Market> Identifier<SubscriptionId> for PublicExchangeSub<Channel, Market>
where
    Channel: AsRef<str>,
    Market: AsRef<str>,
{
    fn id(&self) -> SubscriptionId {
        SubscriptionId::from(format!(
            "{}|{}",
            self.channel.as_ref(),
            self.market.as_ref()
        ))
    }
}

impl<Channel, Market> ExchangeSub for PublicExchangeSub<Channel, Market> {}

/// Defines an exchange specific channel used by an exchange
/// normally used for private endpoit that requires only one
/// ```json
/// PrivateExchangeSub {
///     channel: BybitChannel("execution"),
/// }
/// ```
pub struct PrivateExchangeSub<Channel> {
    pub channel: Channel,
}

impl<Channel> PrivateExchangeSub<Channel>
where
    Channel: AsRef<str>,
{
    /// Construct a new exchange specific [`Self`] with [`Subscription`] provided
    pub fn new<Exchange, Kind>(sub: &Subscription<Exchange, Kind>) -> Self
    where
        Subscription<Exchange, Kind>: Identifier<Channel>,
        Kind: SubKind,
    {
        Self { channel: sub.id() }
    }
}

impl<Channel> Identifier<SubscriptionId> for PrivateExchangeSub<Channel>
where
    Channel: AsRef<str>,
{
    fn id(&self) -> SubscriptionId {
        SubscriptionId::from(self.channel.as_ref())
    }
}

impl<Channel> ExchangeSub for PrivateExchangeSub<Channel> {}
