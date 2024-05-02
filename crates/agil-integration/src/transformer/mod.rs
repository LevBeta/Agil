use serde::Deserialize;

/// A simple transformer trait that takes a Input and converts into a Output
pub trait Transformer {
    type Input: for<'de> Deserialize<'de> + std::fmt::Debug;
    type Output;
    type OutputIter: IntoIterator<Item = Result<Self::Output, Self::Error>>;
    type Error;
    fn transform(&mut self, input: Self::Input) -> Self::OutputIter;
}
