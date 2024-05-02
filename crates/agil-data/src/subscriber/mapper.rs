use crate::{
    exchange::subscription::{PrivateExchangeSub, PublicExchangeSub},
    exchange::Connector,
    subscription::{SubKind, Subscription},
    Identifier,
};
use agil_integration::{instrument::Instrument, websocket::fastws::FastWsMessage};

/// Returns a collections of [`Subscription`]'s into exxchange specific [`SubscriptionMeta`]
pub trait SubscriptionMapper {
    fn map<Exchange, Kind>(
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> (Vec<FastWsMessage>, Vec<Instrument>)
    where
        Exchange: Connector,
        Kind: SubKind,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>;
}

/// Standard [`SubscriptionMapper`] for websockets
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WebSocketSubMapper;

impl SubscriptionMapper for WebSocketSubMapper {
    fn map<Exchange, Kind>(
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> (Vec<FastWsMessage>, Vec<Instrument>)
    where
        Exchange: Connector,
        Kind: SubKind,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        // TODO -> This can be made better, is this actually the better way?
        // First we separete public and private subs into their responsives
        // Vec's, to feed their responsive function into [`Connector`]
        // that will return the messages that need to be sent.

        let mut map: Vec<Instrument> = Vec::new();

        let public_subs = subscriptions
            .iter()
            .filter(|sub| !sub.is_private())
            .map(|sub| {
                map.push(sub.instrument.clone().unwrap());
                PublicExchangeSub::<Exchange::Channel, Exchange::Market>::new(sub)
            })
            .collect::<Vec<PublicExchangeSub<Exchange::Channel, Exchange::Market>>>();

        let private_subs = subscriptions
            .iter()
            .filter(|sub| sub.is_private())
            .map(|sub| PrivateExchangeSub::<Exchange::Channel>::new(sub))
            .collect::<Vec<PrivateExchangeSub<Exchange::Channel>>>();

        let mut msgs: Vec<FastWsMessage> = Exchange::public_requests(public_subs);
        msgs.extend(Exchange::private_requests(private_subs));

        (msgs, map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mapper {
        use super::*;
        use crate::exchange::bybit::spot::BybitSpot;
        use crate::subscription::trade::PublicTrades;
        use crate::subscription::Subscription;
        use agil_integration::instrument::kind::InstrumentKind;
        use agil_integration::instrument::Instrument;
        #[test]
        fn test_subscription_mapper() {
            let instrument = Instrument::new("btc", "usdt", InstrumentKind::Perpetual);
            let sub = Subscription::new(BybitSpot::default(), instrument, PublicTrades);
            let subs = [sub];
            let s = WebSocketSubMapper::map(&subs);
        }
    }
}
