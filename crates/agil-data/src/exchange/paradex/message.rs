use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct ParadexPayload<T> {
    #[serde(alias = "params")]
    pub inner: ParadexInnerPayload<T>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct ParadexInnerPayload<T> {
    pub channel: String,
    pub data: T,
}
