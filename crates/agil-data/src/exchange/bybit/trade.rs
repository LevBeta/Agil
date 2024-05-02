use crate::{
    event::{MarketEvent, MarketIter},
    exchange::bybit::message::PublicBybitPayload,
    exchange::ExchangeId,
    subscription::trade::PublicTrade,
};
use agil_integration::instrument::Side;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Types alias for an [`BybitTrade`]
pub type BybitTrade = PublicBybitPayload<Vec<BybitTradeInner>>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
pub struct BybitTradeInner {
    #[serde(
        alias = "T",
        deserialize_with = "agil_integration::de::de_u64_epoch_ms_as_datetime_utc"
    )]
    pub time: DateTime<Utc>,

    #[serde(rename = "s")]
    pub market: String,

    #[serde(rename = "S")]
    pub side: Side,

    #[serde(alias = "v", deserialize_with = "agil_integration::de::de_str")]
    pub amount: f64,

    #[serde(alias = "p", deserialize_with = "agil_integration::de::de_str")]
    pub price: f64,
}

impl From<(ExchangeId, BybitTrade)> for MarketIter<PublicTrade> {
    fn from((exchange_id, trades): (ExchangeId, BybitTrade)) -> Self {
        let received_time = Utc::now();
        Self(
            trades
                .data
                .into_iter()
                .map(|trade| {
                    Ok(MarketEvent {
                        exchange_time: trade.time,
                        received_time,
                        exchange: exchange_id.clone(),
                        data: PublicTrade {
                            id: trade.market,
                            price: trade.price,
                            amount: trade.amount,
                            side: trade.side,
                        },
                    })
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bybit {
        use super::*;

        #[test]
        fn test_bybit_trade() {
            struct TestCase {
                input: &'static str,
                expected: Result<BybitTradeInner, Box<dyn std::error::Error>>,
            }

            let tests = vec![TestCase {
                input: r#"
                        {
                            "T": 1672304486865,
                            "s": "BTCUSDT",
                            "S": "Buy",
                            "v": "0.001",
                            "p": "16578.50",
                            "L": "PlusTick",
                            "i": "20f43950-d8dd-5b31-9112-a178eb6023af",
                            "BT": false
                        }
                    "#,
                expected: Ok(BybitTradeInner {
                    time: chrono::DateTime::from_timestamp_millis(1672304486865).unwrap(),
                    market: "BTCUSDT".to_string(),
                    side: Side::Buy,
                    amount: 0.001,
                    price: 16578.50,
                }),
            }];

            for (index, test) in tests.into_iter().enumerate() {
                let actual = serde_json::from_str::<BybitTradeInner>(test.input);
                match (actual, test.expected) {
                    (Ok(actual), Ok(expected)) => {
                        assert_eq!(actual, expected, "TC{} failed", index)
                    }
                    (Err(_), Err(_)) => {
                        // Test passed
                    }
                    (actual, expected) => {
                        // Test failed
                        panic!("TC{index} failed because actual != expected. \nActual: {actual:?}\nExpected: {expected:?}\n");
                    }
                }
            }
        }
    }
}
