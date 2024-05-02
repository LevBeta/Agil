use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct PublicBybitPayload<T> {
    #[serde(alias = "topic")]
    pub subsription_id: String,
    // Removed time because it is not used here
    // but can be re-added if we want to have a
    // better options to check market-data lantecy
    //#[serde(alias = "ts")]
    //pub time: u64,
    #[serde(alias = "data")]
    pub data: T,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct PrivateBybitPayload<T> {
    pub data: T,
}
