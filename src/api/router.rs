use axum::Router;

// Api module market_feed
use crate::module::market_feed::{self, state::SharedPriceStore};
// Api module user
use crate::module::user::router as user_router;

pub fn build_router(store: SharedPriceStore) -> Router {
    Router::new()
        .merge(market_feed::router::routes(store))
        .nest("/api/users", user_router::routes())
}
