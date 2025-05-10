use axum::{Router, routing::post};
use crate::module::user::handler;

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(handler::login))
        .route("/register", post(handler::register))
}
