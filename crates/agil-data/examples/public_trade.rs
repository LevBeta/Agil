use agil_data::{
    exchange::bybit::futures::BybitFutures, exchange::paradex::Paradex, stream::StreamBuilder,
    subscription::trade::PublicTrades,
};
use agil_integration::instrument::{kind::InstrumentKind, Instrument};

#[tokio::main]
async fn main() {
    let instrument = Instrument::new("eth", "usd", InstrumentKind::Perpetual);
    let instrument2 = Instrument::new("btc", "usd", InstrumentKind::Perpetual);

    let mut rx = StreamBuilder::new()
        // Single connection for high-volume ticker
        .subscribe(
            [
                (Paradex, instrument, PublicTrades),
                (Paradex, instrument2, PublicTrades),
            ],
            None,
        )
        .init()
        .await;

    while let Some(event) = rx.recv().await {
        println!("{:?}", event);
    }
}
