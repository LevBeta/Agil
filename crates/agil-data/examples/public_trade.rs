use agil_data::{
    exchange::bybit::futures::BybitFutures, stream::StreamBuilder,
    subscription::trade::PublicTrades,
};
use agil_integration::instrument::{kind::InstrumentKind, Instrument};

#[tokio::main]
async fn main() {
    let instrument = Instrument::new("btc", "usdt", InstrumentKind::Perpetual);
    let instrument_2 = Instrument::new("doge", "usdt", InstrumentKind::Perpetual);
    let instrument_3 = Instrument::new("near", "usdt", InstrumentKind::Perpetual);

    let mut rx = StreamBuilder::new()
        // Single connection for high-volume ticker
        .subscribe([(BybitFutures::default(), instrument, PublicTrades)], None)
        // Connection with multiple tickers for low-volume tickers
        .subscribe(
            [
                (BybitFutures::default(), instrument_2, PublicTrades),
                (BybitFutures::default(), instrument_3, PublicTrades),
            ],
            None,
        )
        .init()
        .await;

    while let Some(event) = rx.recv().await {
        println!("{:?}", event);
    }
}
