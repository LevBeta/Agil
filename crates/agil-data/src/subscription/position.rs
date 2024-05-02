use super::SubKind;
use agil_integration::instrument::Side;

/// Agil [`Subscription`][`SubKind`] that yields [`Position`]
#[derive(Debug, Clone)]
pub struct Positions;

impl SubKind for Positions {
    type Event = Position;

    const PRIVATE: bool = true;
}

/// Normalized Agil [`Position`]
#[derive(Debug, Clone)]
pub struct Position {
    pub id: String,
    pub entry_price: f64,
    pub amount: f64,
    pub side: Side,
}
