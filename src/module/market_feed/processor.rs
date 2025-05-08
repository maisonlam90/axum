use serde_json::Value;
use super::event_bus::EventBus;
use super::state::SharedPriceStore;

pub async fn process_message(json: Value, event_bus: &impl EventBus, store: &SharedPriceStore) {
    if let Some(data) = json.get("data") {
        if let (Some(symbol), Some(price)) = (
            data.get("s").and_then(|v| v.as_str()),
            data.get("p").and_then(|v| v.as_str()),
        ) {
            {
                let mut lock = store.lock().await;
                lock.insert(symbol.to_string(), price.to_string());
            }

            let event = format!(r#"{{"type":"MarketPrice","symbol":"{}","price":{}}}"#, symbol, price);
            let _ = event_bus.publish("market.price", &event).await;
        }
    }
}
