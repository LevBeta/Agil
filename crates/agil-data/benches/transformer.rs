use agil_data::exchange::bybit::trade::BybitTrade;
use agil_data::exchange::bybit::Bybit;
use agil_data::subscription::trade::PublicTrades;
use agil_data::transformer::{ExchangeTransformer, StatelessTransformer};
use agil_integration::transformer::Transformer;
use agil_integration::websocket::message::{StreamParser, WebSocketParser};
use agil_integration::websocket::{ExchangeStream, WsError, WsMessage, WsStream};
use criterion::{
    async_executor::FuturesExecutor, black_box, criterion_group, criterion_main, Criterion,
};
use futures::stream::Stream;
use futures::StreamExt;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;

fn criterion_benchmark(c: &mut Criterion) {
    pub type ExchangeWsStream<Transformer> =
        ExchangeStream<Transformer, WebSocketParser, MockStream>;

    let mut transformer = StatelessTransformer::<Bybit, PublicTrades, BybitTrade>::new();
    //let y = Transformer::Input;
    let mut input = r#"
    {
    "topic": "publicTrade.BTCUSDT",
    "type": "snapshot",
    "ts": 1672304486868,
    "data": [
        {
            "T": 1672304486865,
            "s": "BTCUSDT",
            "S": "Buy",
            "v": "0.001",
            "p": "16578.50",
            "L": "PlusTick",
            "i": "20f43950-d8dd-5b31-9112-a178eb6023af",
            "BT": false
        }
    ]
}
                    "#;

    let input_ws: Result<WsMessage, WsError> = Ok(input.into());

    c.bench_function("transformer", |b| {
        b.iter(|| {
            let input_ws: Result<WsMessage, WsError> = Ok(input.into());
            let exchange_message = match WebSocketParser::parse::<BybitTrade>(input_ws) {
                Some(Ok(exchange_message)) => exchange_message,
                _ => todo!(),
            };

            let x = transformer.transform(black_box(exchange_message));
        })
    });

    struct MockStream {
        messages: VecDeque<Result<WsMessage, WsError>>,
    }

    impl Stream for MockStream {
        type Item = Result<WsMessage, WsError>;

        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.messages.pop_front())
        }
    }

    impl MockStream {
        fn new(x: VecDeque<Result<WsMessage, WsError>>) -> Self {
            Self { messages: x }
        }
    }

    c.bench_function("ews", |b| {
        b.to_async(FuturesExecutor).iter(|| async move {
            println!("{:?}", chrono::Utc::now());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
