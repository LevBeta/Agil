use super::Paradex;
use crate::{
    subscription::{SubKind, Subscription},
    Identifier,
};

/// Type that defines how to translate a Agil [`Subscription`] into a [`Paradex`]
/// market that can be subscriped to
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ParadexMarket(pub String);

impl<Kind> Identifier<ParadexMarket> for Subscription<Paradex, Kind>
where
    Kind: SubKind,
{
    fn id(&self) -> ParadexMarket {
        let instrument = match &self.instrument {
            Some(instrument) => instrument,
            None => todo!(),
        };
        ParadexMarket(format!("{:?}-{:?}-PERP", instrument.base, instrument.quote).to_uppercase())
    }
}

impl AsRef<str> for ParadexMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
