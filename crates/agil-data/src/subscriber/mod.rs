use crate::{
    error::AgilDataError,
    exchange::Connector,
    subscriber::{
        mapper::{SubscriptionMapper, WebSocketSubMapper},
        validator::SubscriptionValidator,
    },
    subscription::{SubKind, Subscription},
    Identifier,
};
use agil_integration::{
    account::ApiKey,
    websocket::fastws::{connect, FastWebSocket},
};
use trait_variant::make;

/// [`SubscriptionMapper`] implementatiosn defines how to map a collection
/// of [`Subscription`] into exchange specifiec subscriptions messages
pub mod mapper;

/// [`SubscriptionValidator`] implementation defines how to validate if [`Subscription`]'s
/// where succeful
pub mod validator;

/// Defines how to connect to a Websocket and subscribe to feeds using a [`SubcsriptionMapper`]
#[make(Send)]
pub trait Subscriber {
    type SubMapper: SubscriptionMapper;

    async fn subscribe<Exchange, Kind, Key>(
        subscriptions: &[Subscription<Exchange, Kind>],
        api_key: Option<<Exchange as Connector>::Key>,
    ) -> Result<FastWebSocket, AgilDataError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        //Key: ApiKey + Send + Sync,
        <Exchange as Connector>::Key: ApiKey + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>;
}

/// A standard [`Subscriber`] for [`WebSocket`] suitable for most exchanges.
#[derive(Copy, Clone)]
pub struct WebSocketSubscriber;

impl Subscriber for WebSocketSubscriber {
    type SubMapper = WebSocketSubMapper;

    async fn subscribe<Exchange, Kind, Key>(
        subscriptions: &[Subscription<Exchange, Kind>],
        api_key: Option<<Exchange as Connector>::Key>,
    ) -> Result<FastWebSocket, AgilDataError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        <Exchange as Connector>::Key: ApiKey + Send + Sync,
        //Key: ApiKey + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        let url = Exchange::url();

        // Connect to a exchange
        let mut websocket = connect(url).await;

        //if api_key.is_some() {
        //    let msg = Exchange::private_connect(api_key.unwrap());
        //println!("{:?}", msg);
        //    websocket.write_frame(msg).await.unwrap();
        //}
        if api_key.is_some() {
            let msg = Exchange::private_connect(api_key.unwrap());
            websocket.write_frame(msg).await.unwrap();
        }

        // Maps &[Subscription<Exchange, Kind>] into a [`WsMessage`]
        let (msgs, map) = Self::SubMapper::map::<Exchange, Kind>(subscriptions);

        // Sends subscriptions over socket
        for subscription in msgs {
            websocket.write_frame(subscription).await.unwrap();
        }

        Exchange::SubValidator::validate::<Exchange, Kind>(&mut websocket, map).await?;

        Ok(websocket)
    }
}
