use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Normalized Agil L1 Orderbook implementation [`OrderBookL1`]
pub mod l1;

/// Normalized Agil OrderBook [`Level`]
#[derive(Clone, Copy, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Level {
    pub price: f64,
    pub amount: f64,
}

impl<T> From<(T, T)> for Level
where
    T: Into<f64>,
{
    fn from((price, amount): (T, T)) -> Self {
        Self::new(price, amount)
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Ordering::Equal = self.price.partial_cmp(&other.price)? {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}

impl Eq for Level {}

impl Level {
    pub fn new<T>(price: T, amount: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            price: price.into(),
            amount: amount.into(),
        }
    }
}

/// Calculate the mid price of two prices
pub fn mid_price(best_bid: f64, best_ask: f64) -> f64 {
    (best_bid + best_ask) / 2.0
}
