pub mod binance_ws;
pub mod processor;
pub mod handler;
pub mod router;
mod event_bus;
pub mod state;

use tokio::sync::mpsc;
use state::{new_price_store, SharedPriceStore};
use event_bus::get_event_bus;

pub async fn start_market_feed() -> SharedPriceStore {
    let store = new_price_store();
    let (tx, mut rx) = mpsc::channel(100);
    let event_bus = get_event_bus();
    let store_clone = store.clone();

    tokio::spawn(async move {
        binance_ws::start_binance_ws(tx).await;
    });

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            processor::process_message(msg, &event_bus, &store_clone).await;
        }
    });

    store
}
