use super::Paradex;
use crate::{
    subscription::{trade::PublicTrades, Subscription},
    Identifier,
};

/// Type that defines how to translate a Agil [`Subscription`] into a [`Bybit`]
/// channel that can be subscriped to
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ParadexChannel(pub &'static str);

impl ParadexChannel {
    /// [`Paradex`] real-time public trades channel
    pub const TRADES: Self = Self("trades");
}

impl Identifier<ParadexChannel> for Subscription<Paradex, PublicTrades> {
    fn id(&self) -> ParadexChannel {
        ParadexChannel::TRADES
    }
}

impl AsRef<str> for ParadexChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
