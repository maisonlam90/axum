use axum::{Json, response::IntoResponse};
use std::collections::HashMap;

pub async fn get_price() -> impl IntoResponse {
    let mut prices = HashMap::new();
    prices.insert("BTCUSDT", "62450.00");
    prices.insert("ETHUSDT", "3050.23");
    Json(prices)
}
