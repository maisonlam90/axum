use axum::Router;
use crate::module::market_feed::{self, state::SharedPriceStore};

pub fn build_router(store: SharedPriceStore) -> Router {
    Router::new().merge(market_feed::router::routes(store))
}
