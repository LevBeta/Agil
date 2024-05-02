use crate::error::AgilDataError;
use agil_integration::Validator;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct BybitResponse {
    pub success: bool,
    #[serde(default)]
    pub ret_msg: BybitReturnMessage,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum BybitReturnMessage {
    #[serde(alias = "")]
    None,
    #[serde(alias = "pong")]
    Pong,
    #[serde(alias = "subscribe")]
    Subscribe,
    #[serde(alias = "auth")]
    Auth,
}

impl Default for BybitReturnMessage {
    fn default() -> Self {
        Self::None
    }
}

impl Validator for BybitResponse {
    type Error = AgilDataError;

    fn validate(self) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match self.ret_msg {
            BybitReturnMessage::None | BybitReturnMessage::Subscribe | BybitReturnMessage::Auth => {
                if self.success {
                    Ok(self)
                } else {
                    // This error should be changed, to support responding
                    // with the specifiec subscription that failed
                    Err(Self::Error::FailedSubscription)
                }
            }
            _ => Err(Self::Error::OutOfSequenceMessage),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod bybit {
        use super::*;

        #[test]
        fn test_bybit_sub_response() {
            struct TestCase {
                input: &'static str,
                expected: Result<BybitResponse, Box<dyn std::error::Error>>,
            }

            let cases = vec![TestCase {
                // TC0: input response is Subscribed
                input: r#"
                        {
                            "success": true,
                            "ret_msg": "subscribe",
                            "conn_id": "2324d924-aa4d-45b0-a858-7b8be29ab52b",
                            "req_id": "10001",
                            "op": "subscribe"
                        }
                    "#,
                expected: Ok(BybitResponse {
                    success: true,
                    ret_msg: BybitReturnMessage::Subscribe,
                }),
            }];

            for (index, test) in cases.into_iter().enumerate() {
                let actual = serde_json::from_str::<BybitResponse>(test.input);
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
