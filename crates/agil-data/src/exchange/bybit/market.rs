use super::Bybit;
use crate::{
    subscription::{SubKind, Subscription},
    Identifier,
};

/// Type that defines how to translate a Agil [`Subscription`] into a [`Bybit`]
/// market that can be subscriped to
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BybitMarket(pub String);

impl<Kind, Server> Identifier<BybitMarket> for Subscription<Bybit<Server>, Kind>
where
    Kind: SubKind,
{
    fn id(&self) -> BybitMarket {
        let instrument = match &self.instrument {
            Some(instrument) => instrument,
            None => return BybitMarket(String::from("")),
        };
        BybitMarket(format!("{:?}{:?}", instrument.base, instrument.quote).to_uppercase())
    }
}

impl AsRef<str> for BybitMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
