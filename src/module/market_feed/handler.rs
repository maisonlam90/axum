use axum::{Json, response::IntoResponse};
use std::collections::HashMap;
use super::state::SharedPriceStore;

pub async fn get_price(store: SharedPriceStore) -> impl IntoResponse {
    let lock = store.lock().await;
    Json(lock.clone())
}
