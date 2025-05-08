use serde_json::Value;
use super::event_bus::EventBus;

pub async fn process_message(json: Value, event_bus: &impl EventBus) {
    if let Some(data) = json.get("data") {
        let price = data.get("p").and_then(|v| v.as_str()).unwrap_or_default();
        let symbol = data.get("s").and_then(|v| v.as_str()).unwrap_or_default();

        let event = format!(r#"{{"type":"MarketPrice","symbol":"{}","price":{}}}"#, symbol, price);
        event_bus.publish("market.price", &event).await.unwrap();
    }
}
