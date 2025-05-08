use axum::{Router, routing::get};
use super::{handler, state::SharedPriceStore};

pub fn routes(store: SharedPriceStore) -> Router {
    Router::new().route("/api/price", get({
        let store = store.clone();
        move || handler::get_price(store)
    }))
}
