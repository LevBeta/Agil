//! #Agil-Integration
//! High performance framework for flexible creation of trading systems
//!
//! Utilised by other Agil trading ecosystem crates to build exchange integrations
//!
//! CHECK "TODO's" if you want some nice first issues

/// [´Instrument´] related data structures.
pub mod instrument;

/// Websocket helpers
pub mod websocket;

/// Utilities for fast deserialization.
pub mod de;

/// Strecutres and traits to help transforming data/deserializing
pub mod transformer;

/// Utilities that can be used that envolves net
pub mod net;

/// Some account-realated strecutes
pub mod account;

/// Simple trait validator that can validate any type of input
/// TODO: Move this to other file
pub trait Validator {
    type Error;

    fn validate(self) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Basically keeps a Id for a subscription
/// TODO: Move this to other file
#[derive(Clone)]
pub struct SubscriptionId(pub String);

impl<S> From<S> for SubscriptionId
where
    S: Into<String>,
{
    fn from(input: S) -> Self {
        Self(input.into())
    }
}
