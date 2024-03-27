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

/// [´Instrument´] related data structures.
pub mod instrument;

/// Websocket helpers
pub mod websocket;

/// Utilities for fast deserialization.
pub mod de;

pub trait Validator {
    type Error;

    fn validate(self) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
