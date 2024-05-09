use crate::{subscription::book::Level, SubKind};
use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct OrderBooksL1;

impl SubKind for OrderBooksL1 {
    type Event = OrderBookL1;

    const PRIVATE: bool = false;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct OrderBookL1 {
    pub last_update_time: DateTime<Utc>,
    pub best_bid: Level,
    pub best_ask: Level,
}
