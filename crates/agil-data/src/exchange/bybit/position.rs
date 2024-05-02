use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{bybit::message::PrivateBybitPayload, ExchangeId},
    subscription::position::Position,
};
use agil_integration::instrument::Side;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Types alias for an [`BybitPosition`]
pub type BybitPosition = PrivateBybitPayload<Vec<BybitPositionInner>>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
pub struct BybitPositionInner {
    #[serde(
        alias = "updatedTime",
        deserialize_with = "agil_integration::de::de_u64_epoch_ms_as_datetime_utc"
    )]
    pub updated_time: DateTime<Utc>,

    #[serde(rename = "symbol")]
    pub market: String,

    #[serde(rename = "side")]
    pub side: Side,

    #[serde(rename = "size", deserialize_with = "agil_integration::de::de_str")]
    pub amount: f64,

    #[serde(
        rename = "entryPrice",
        deserialize_with = "agil_integration::de::de_str"
    )]
    pub entry_price: f64,
}

impl From<(ExchangeId, BybitPosition)> for MarketIter<Position> {
    fn from((exchange_id, positions): (ExchangeId, BybitPosition)) -> Self {
        let received_time = Utc::now();
        Self(
            positions
                .data
                .into_iter()
                .map(|position| {
                    Ok(MarketEvent {
                        exchange_time: received_time,
                        received_time,
                        exchange: exchange_id.clone(),
                        data: Position {
                            id: position.market,
                            entry_price: position.entry_price,
                            amount: position.amount,
                            side: position.side,
                        },
                    })
                })
                .collect(),
        )
    }
}
