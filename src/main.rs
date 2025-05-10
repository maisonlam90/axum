use axum::{Router, routing::get, Extension};
use dotenvy::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
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
    dotenv().ok(); // Load biến môi trường từ file .env

    // Kết nối tới cơ sở dữ liệu PostgreSQL
    let db_url = env::var("DATABASE_URL").expect("⚠️ DATABASE_URL chưa được cấu hình");
    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("❌ Không thể kết nối DB");

    // 👉 Khởi động WebSocket Binance trong task nền
    module::market_feed::start_market_feed().await;
    let store = module::market_feed::start_market_feed().await;

    // Thiết lập CORS cho phép mọi origin, method và header
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Bọc kết nối DB vào Arc để chia sẻ giữa các handler thông qua Extension
    let shared_db = db_pool;

    // Tạo router chính và gắn middleware CORS và DB
    let app = api::router::build_router(store)
        .layer(cors)
        .layer(Extension(shared_db.clone()));

    // Cấu hình cổng và địa chỉ để server lắng nghe kết nối
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Axum khởi động tại http://{}", addr);

    // Khởi chạy server Axum và xử lý lỗi nếu có
    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await {
        eprintln!("❌ Lỗi khi chạy server: {}", e);
    }
}
