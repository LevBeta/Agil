use crate::error::AgilDataError;
use agil_integration::Validator;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct ParadexResponse {
    pub error: Option<ParadexResponseError>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct ParadexResponseError {
    pub code: f64,
    pub message: String,
    pub data: String,
}

impl Validator for ParadexResponse {
    type Error = AgilDataError;

    fn validate(self) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match self.error {
            None => Ok(self),
            Some(_) => Err(Self::Error::FailedSubscription),
        }
    }
}
