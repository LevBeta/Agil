use crate::instrument::kind::InstrumentKind;
use serde::{Deserialize, Serialize};
/// Agil representation of various market kinds
pub mod kind;

/// Agil representation of a currency symbol
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Symbol(String);

impl Symbol {
    pub fn new<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        Self(input.into().to_lowercase())
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<S> From<S> for Symbol
where
    S: Into<String>,
{
    fn from(input: S) -> Self {
        Symbol::new(input)
    }
}

/// Agil representation of a instrument
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Instrument {
    pub base: Symbol,
    pub quote: Symbol,
    pub kind: InstrumentKind,
}

impl Instrument {
    pub fn new<S>(base: S, quote: S, kind: InstrumentKind) -> Self
    where
        S: Into<Symbol>,
    {
        Self {
            base: base.into(),
            quote: quote.into(),
            kind,
        }
    }
}

impl<S> From<(S, S, InstrumentKind)> for Instrument
where
    S: Into<Symbol>,
{
    fn from((base, quote, kind): (S, S, InstrumentKind)) -> Self {
        Instrument::new(base, quote, kind)
    }
}

/// [`Side`] of a trade or position - Buy or Sell.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Side {
    #[serde(alias = "buy", alias = "BUY", alias = "b")]
    Buy,
    #[serde(alias = "sell", alias = "SELL", alias = "s")]
    Sell,
}
