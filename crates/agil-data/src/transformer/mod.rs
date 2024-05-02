use crate::{
    error::AgilDataError,
    event::{MarketEvent, MarketIter},
    exchange::Connector,
    exchange::ExchangeId,
    SubKind,
};
use agil_integration::transformer::Transformer;
use serde::de::Deserialize;
use std::marker::PhantomData;

/// Defines how to construct a [`Transformer`]
///
/// This needs eventually be re-done better
/// to handle orderbook transformer etc.
pub trait ExchangeTransformer<Exchange, Kind>
where
    Kind: SubKind,
    Self: Transformer<Output = MarketEvent<Kind::Event>, Error = AgilDataError> + Sized,
{
    fn new() -> Self;
}

/// A simple [`Transformer`] implementation that takes a
/// input and returns a output, that doesn't hold any
/// inner state
pub struct StatelessTransformer<Exchange, Kind, Input> {
    phantom: PhantomData<(Exchange, Kind, Input)>,
}

impl<Exchange, Kind, Input> ExchangeTransformer<Exchange, Kind>
    for StatelessTransformer<Exchange, Kind, Input>
where
    Exchange: Connector,
    Kind: SubKind,
    Input: for<'de> Deserialize<'de> + std::fmt::Debug,
    MarketIter<Kind::Event>: From<(ExchangeId, Input)>,
{
    fn new() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

impl<Exchange, Kind, Input> Transformer for StatelessTransformer<Exchange, Kind, Input>
where
    Exchange: Connector,
    Kind: SubKind,
    Input: for<'de> Deserialize<'de> + std::fmt::Debug,
    MarketIter<Kind::Event>: From<(ExchangeId, Input)>,
{
    type Input = Input;
    type Output = MarketEvent<Kind::Event>;
    type OutputIter = Vec<Result<Self::Output, Self::Error>>;
    type Error = AgilDataError;
    fn transform(&mut self, input: Self::Input) -> Self::OutputIter {
        MarketIter::<Kind::Event>::from((Exchange::ID, input)).0
    }
}
