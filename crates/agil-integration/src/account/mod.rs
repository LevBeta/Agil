/// Defines a trait for signing operations
pub trait Signer {
    type Input;
    type Output;
    type Error;

    /// Method to sign data
    fn sign(data: Self::Input) -> Result<Self::Output, Self::Error>;
}

/// Defines a trait for API key handling
pub trait ApiKey {
    type Signer: Signer;

    /// Calls [`Signer`] to sign a message
    fn sign(self) -> Result<<Self::Signer as Signer>::Output, <Self::Signer as Signer>::Error>;
}
