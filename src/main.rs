use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use sqlx::PgPool;

mod core;
mod infra;
mod api;
mod module;
mod tenant_router;
mod command_bus;
mod query_bus;
mod event_handler;

use tower_http::cors::{CorsLayer, Any};




#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("⚠️ DATABASE_URL chưa được cấu hình");
    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("❌ Không thể kết nối DB");

    // 👉 Khởi động WebSocket Binance trong task nền
    module::market_feed::start_market_feed().await;
    let store = module::market_feed::start_market_feed().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = api::router::build_router(store).layer(cors);

    

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("🚀 Axum khởi động tại http://{}", addr);

    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await {
        eprintln!("❌ Lỗi khi chạy server: {}", e);
    }
}
