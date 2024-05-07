use agil_integration::account::{ApiKey, Signer};

#[derive(Clone, Debug)]
pub struct ParadexApiKey;

impl ApiKey for ParadexApiKey {
    type Signer = ParadexSigner;

    fn sign(self) -> Result<<Self::Signer as Signer>::Output, <Self::Signer as Signer>::Error> {
        todo!()
    }
}

pub struct ParadexSigner;

impl Signer for ParadexSigner {
    type Input = ParadexApiKey;
    type Output = String;
    type Error = std::io::Error;

    fn sign(data: Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
