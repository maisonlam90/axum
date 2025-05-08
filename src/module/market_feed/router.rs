use axum::{Router, routing::get};
use super::handler;

pub fn routes() -> Router {
    Router::new().route("/api/price", get(handler::get_price))
}
