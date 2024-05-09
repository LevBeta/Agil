use crate::{error::AgilDataError, exchange::ExchangeId, subscription::trade::PublicTrade};
use chrono::DateTime;
use chrono::Utc;

/// Type containing a collection of [`MarketEvent<T>`].
pub struct MarketIter<T>(pub Vec<Result<MarketEvent<T>, AgilDataError>>);

impl<T> FromIterator<Result<MarketEvent<T>, AgilDataError>> for MarketIter<T> {
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: IntoIterator<Item = Result<MarketEvent<T>, AgilDataError>>,
    {
        Self(iter.into_iter().collect())
    }
}

/// Normalized [`MarketEvent<T>`] wraps a data variant in metadata
#[derive(Clone, Debug)]
pub struct MarketEvent<T> {
    pub received_time: DateTime<Utc>,
    pub exchange_time: DateTime<Utc>,
    pub exchange: ExchangeId,
    pub data: T,
}
