use crate::{
    error::AgilDataError,
    event::MarketEvent,
    exchange::{Connector, StreamSelector, TransformerSelector},
    stream::consume::consume,
    subscription::{SubKind, Subscription},
    ExchangeTransformer, Identifier,
};
use agil_integration::account::ApiKey;
use tokio::sync::mpsc;
pub mod consume;

use std::{future::Future, pin::Pin};

pub(crate) type SubscribeFuture = Pin<Box<dyn Future<Output = Result<(), AgilDataError>>>>;

pub struct StreamBuilder<Kind>
where
    Kind: SubKind,
{
    pub futures: Vec<SubscribeFuture>,
    pub rx: mpsc::UnboundedReceiver<MarketEvent<Kind::Event>>,
    pub tx: mpsc::UnboundedSender<MarketEvent<Kind::Event>>,
}

impl<Kind> StreamBuilder<Kind>
where
    Kind: SubKind,
{
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<MarketEvent<Kind::Event>>();
        Self {
            futures: Vec::new(),
            tx,
            rx,
        }
    }

    pub fn subscribe<SubIter, Sub, Exchange, Key>(
        mut self,
        subscriptions: SubIter,
        api_key: Option<Key>,
    ) -> Self
    where
        SubIter: IntoIterator<Item = Sub>,
        Sub: Into<Subscription<Exchange, Kind>>,
        Exchange: Connector
            + StreamSelector<Kind, Key>
            + TransformerSelector<Kind>
            + Send
            + Sync
            + 'static,
        Key: ApiKey + Send + Sync + Clone + 'static,
        Kind: SubKind + Send + Sync + 'static,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
        Kind::Event: Send,
        <Exchange as TransformerSelector<Kind>>::Transformer: std::marker::Send,
    {
        let mut subscriptions = subscriptions.into_iter().map(Sub::into).collect::<Vec<_>>();

        let channel = self.tx.clone();

        self.futures.push(Box::pin(async move {
            let transformer = Exchange::Transformer::new();
            tokio::spawn(consume(subscriptions, api_key, transformer, channel));
            Ok(())
        }));

        self
    }

    pub async fn init(self) -> mpsc::UnboundedReceiver<MarketEvent<Kind::Event>> {
        let _ = futures::future::try_join_all(self.futures).await;
        self.rx
    }
}
