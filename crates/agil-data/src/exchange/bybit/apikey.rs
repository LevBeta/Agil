use agil_integration::account::{ApiKey, Signer};

#[derive(Clone, Debug)]
pub struct BybitApiKey {
    pub api_key: String,
    pub secret_key: String,
}

impl ApiKey for BybitApiKey {
    type Signer = BybitSigner;

    fn sign(self) -> Result<<Self::Signer as Signer>::Output, <Self::Signer as Signer>::Error> {
        Self::Signer::sign(self)
    }
}

impl<S> From<(S, S)> for BybitApiKey
where
    S: Into<String>,
{
    fn from((api_key, secret_key): (S, S)) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }
}

pub struct BybitSigner {}

impl Signer for BybitSigner {
    type Input = BybitApiKey;
    /// Returns signaure and expiry time
    type Output = (String, u64);
    type Error = std::io::Error;

    fn sign(_data: Self::Input) -> Result<Self::Output, Self::Error> {
        todo!();
        //let expires = (time::SystemTime::now()
        //    .duration_since(time::UNIX_EPOCH)
        //    .unwrap()
        //    .as_secs()
        //    + 10)
        //    * 1000;
        //let val = format!("GET/realtime{}", expires);
        //type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        //let mut hmac = HmacSha256::new_from_slice(&data.secret_key.as_bytes()).unwrap();
        //let signature = hmac::Key::new(hmac::HMAC_SHA256, &data.secret_key.as_bytes());
        //let sig_bytes = hmac::sign(&signature, val.as_bytes());

        //let signature_hex = hex::encode(sig_bytes.as_ref());
        //Ok((signature_hex, expires))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_test() {
        let x = BybitApiKey {
            api_key: "Ola".to_string(),
            secret_key: "Adeuts".to_string(),
        };

        println!("{:?}", x.sign());
    }
}
