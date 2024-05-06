use agil_integration::instrument::Instrument;

pub mod trade;

pub mod position;

/// Subsciptions to describe to a [`SubKind`] for a particular exchange
pub struct Subscription<Exchange, Kind>
where
    Kind: SubKind,
{
    pub exchange: Exchange,
    pub instrument: Option<Instrument>,
    pub kind: Kind,
}

impl<Exchange, Kind> Subscription<Exchange, Kind>
where
    Kind: SubKind,
{
    pub fn new<I>(exchange: Exchange, instrument: I, kind: Kind) -> Self
    where
        I: Into<Option<Instrument>>,
    {
        Self {
            exchange,
            instrument: instrument.into(),
            kind,
        }
    }

    pub fn is_private(&self) -> bool {
        Kind::PRIVATE
    }
}

/// Defines a type of [`Subscription`]
/// [`Self::Event`] yields a output
/// [`Self::PRIVATE`] holds a information to know if the Subscription is private or not
pub trait SubKind
where
    Self: std::fmt::Debug + Clone,
{
    type Event: std::fmt::Debug;
    const PRIVATE: bool;
}

impl<Exchange, Kind> From<(Exchange, Kind)> for Subscription<Exchange, Kind>
where
    Kind: SubKind,
{
    fn from((exchange, kind): (Exchange, Kind)) -> Self {
        Self::new(exchange, None, kind)
    }
}

impl<Exchange, I, Kind> From<(Exchange, I, Kind)> for Subscription<Exchange, Kind>
where
    Kind: SubKind,
    I: Into<Option<Instrument>>,
{
    fn from((exchange, instrument, kind): (Exchange, I, Kind)) -> Self {
        Self::new(exchange, instrument, kind)
    }
}
