// binance_ws.rs
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio::sync::mpsc::Sender;
use serde_json::Value;
use url::Url;

pub async fn start_binance_ws(tx: Sender<Value>) {
    let url = Url::parse("wss://stream.binance.com:9443/stream?streams=btcusdt@trade/ethusdt@trade").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                if let Ok(json) = serde_json::from_str::<Value>(&msg.to_string()) {
                    let _ = tx.send(json).await;
                }
            }
        }
    }
}
