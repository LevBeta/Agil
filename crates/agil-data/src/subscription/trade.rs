use super::SubKind;
use agil_integration::instrument::Side;

/// Agil [`Subscription`][`SubKind`] that yields [`PublicTrade`]
#[derive(Debug, Clone)]
pub struct PublicTrades;

impl SubKind for PublicTrades {
    type Event = PublicTrade;

    const PRIVATE: bool = false;
}

/// Normalized Agil [`PublicTrade`]
#[derive(Debug, Clone)]
pub struct PublicTrade {
    pub id: String,
    pub price: f64,
    pub amount: f64,
    pub side: Side,
}
