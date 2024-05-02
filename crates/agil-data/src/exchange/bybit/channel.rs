use super::Bybit;
use crate::{
    subscription::{position::Positions, trade::PublicTrades, Subscription},
    Identifier,
};

/// Type that defines how to translate a Agil [`Subscription`] into a [`Bybit`]
/// channel that can be subscriped to
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BybitChannel(pub &'static str);

impl BybitChannel {
    /// [`Bybit`] real-time public trades channel
    pub const TRADES: Self = Self("publicTrade");

    /// [`Bybit`] real-time private position channel;
    pub const POSITIONS: Self = Self("positions");
}

impl<Server> Identifier<BybitChannel> for Subscription<Bybit<Server>, PublicTrades> {
    fn id(&self) -> BybitChannel {
        BybitChannel::TRADES
    }
}

impl<Server> Identifier<BybitChannel> for Subscription<Bybit<Server>, Positions> {
    fn id(&self) -> BybitChannel {
        BybitChannel::POSITIONS
    }
}

impl AsRef<str> for BybitChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
