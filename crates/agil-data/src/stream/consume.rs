use crate::{
    error::AgilDataError,
    exchange::{Connector, StreamSelector},
    subscription::{SubKind, Subscription},
    ExchangeTransformer, Identifier, MarketEvent, MarketStream,
};
use agil_integration::{
    account::ApiKey,
    websocket::{fastws::message::FastWebSocketParser, message::StreamParser},
};
use tokio::sync::mpsc;

/// Defualt starting reconnectiong backoff to the exchange
pub const STARTING_RECONNECTING_BACKOFF_MS: u64 = 250;

pub async fn consume<Exchange, Kind, Transformer, Key>(
    subscriptions: Vec<Subscription<Exchange, Kind>>,
    api_key: Option<Key>,
    mut transformer: Transformer,
    exchange_tx: mpsc::UnboundedSender<MarketEvent<Kind::Event>>,
) -> AgilDataError
where
    Exchange: Connector + StreamSelector<Kind, Key>,
    Kind: SubKind,
    Transformer: ExchangeTransformer<Exchange, Kind> + Send,
    Key: ApiKey + Clone,
    Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
{
    // Holds loop retry state
    let mut attempt: u8 = 0;
    let backoff_ms: u64 = STARTING_RECONNECTING_BACKOFF_MS;
    loop {
        attempt += 1;

        let mut stream = match Exchange::Stream::init(&subscriptions, api_key.clone()).await {
            Ok(stream) => {
                attempt = 0;
                stream
            }
            Err(err) => {
                if attempt == 2 {
                    return err;
                }
                continue;
            }
        };

        // This is retuning a tuple with a ws stream and a transformer, which we can use
        // instead of the horrible way that is using rn, where we create it on the stream builder
        // and pass it as a argument to here, which was made just to be a placeholder for some time
        // let mut ws_stream = stream;

        // TODO: This should be re-done to make it better
        // Also isn't using any of errors returned, which is horrible
        while let Ok(msg) = stream.read_frame().await {
            let exchange_message = match FastWebSocketParser::parse::<Transformer::Input>(Ok(msg)) {
                Some(Ok(exchange_message)) => exchange_message,
                Some(Err(_)) => continue,
                None => continue,
            };

            transformer
                .transform(exchange_message)
                .into_iter()
                .for_each(|output_result| {
                    let _ = exchange_tx.send(output_result.unwrap());
                });
        }

        tokio::time::sleep(std::time::Duration::from_millis(backoff_ms)).await;
    }
}
