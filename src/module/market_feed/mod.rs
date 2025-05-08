pub mod binance_ws;
pub mod processor;
pub mod handler;
pub mod router;
mod event_bus;

use tokio::sync::mpsc;
use event_bus::get_event_bus;

pub async fn start_market_feed() {
    let (tx, mut rx) = mpsc::channel(100);
    let event_bus = get_event_bus();

    tokio::spawn(async move {
        binance_ws::start_binance_ws(tx).await;
    });

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            processor::process_message(msg, &event_bus).await;
        }
    });
}
