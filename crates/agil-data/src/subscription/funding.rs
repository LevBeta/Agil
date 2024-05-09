use super::SubKind;
use serde::{Deserialize, Serialize};

/// Agil [`Subscription`][`SubKind`] that yields [`Funding`]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Fundings;

impl SubKind for Fundings {
    type Event = Funding;

    const PRIVATE: bool = false;
}

/// Normalized Agil [`Funding`]
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Funding {
    pub funding_rate: f64,
}

impl Ord for Funding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.funding_rate
            .partial_cmp(&other.funding_rate)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Funding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.funding_rate.partial_cmp(&other.funding_rate)
    }
}

impl Eq for Funding {}
