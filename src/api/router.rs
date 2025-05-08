use axum::Router;
use crate::module::market_feed;

pub fn build_router() -> Router {
    Router::new().merge(market_feed::router::routes())
}
