use crate::{
    event::{MarketEvent, MarketIter},
    exchange::paradex::message::ParadexPayload,
    exchange::ExchangeId,
    subscription::trade::PublicTrade,
};
use agil_integration::instrument::Side;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Type alias for an [`ParadexTrade`]
pub type ParadexTrade = ParadexPayload<ParadexTradeInner>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
pub struct ParadexTradeInner {
    #[serde(
        alias = "created_at",
        deserialize_with = "agil_integration::de::de_u64_epoch_ms_as_datetime_utc"
    )]
    pub time: DateTime<Utc>,

    #[serde(alias = "market")]
    market: String,

    #[serde(rename = "side")]
    pub side: Side,

    #[serde(alias = "size", deserialize_with = "agil_integration::de::de_str")]
    pub amount: f64,

    #[serde(alias = "price", deserialize_with = "agil_integration::de::de_str")]
    pub price: f64,
}

impl From<(ExchangeId, ParadexTrade)> for MarketIter<PublicTrade> {
    fn from((exchange_id, trade): (ExchangeId, ParadexTrade)) -> Self {
        let received_time = Utc::now();
        Self(vec![Ok(MarketEvent {
            exchange_time: trade.inner.data.time,
            received_time,
            exchange: exchange_id.clone(),
            data: PublicTrade {
                id: trade.inner.data.market,
                price: trade.inner.data.price,
                amount: trade.inner.data.amount,
                side: trade.inner.data.side,
            },
        })])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod paradex {
        use super::*;

        #[test]
        fn test_paradex_trade() {
            struct TestCase {
                input: &'static str,
                expected: Result<ParadexTradeInner, Box<dyn std::error::Error>>,
            }

            let tests = vec![TestCase {
                input: r#"
                        {
                            "created_at": 1696291200000,
                            "id": "12345643",
                            "market": "string",
                            "price": "30001.2",
                            "side": "BUY",
                            "size": "0.01",
                            "trade_type": "FILL"
                        } "#,
                expected: Ok(ParadexTradeInner {
                    time: chrono::DateTime::from_timestamp_millis(1696291200000).unwrap(),
                    market: "string".to_string(),
                    side: Side::Buy,
                    amount: 0.01,
                    price: 30001.2,
                }),
            }];

            for (index, test) in tests.into_iter().enumerate() {
                let actual = serde_json::from_str::<ParadexTradeInner>(test.input);
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
