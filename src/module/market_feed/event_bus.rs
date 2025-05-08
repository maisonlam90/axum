// module/market_feed/event_bus.rs

use async_trait::async_trait;

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, topic: &str, payload: &str) -> anyhow::Result<()>;
}

// Dummy event bus: log ra stdout
pub struct LocalEventBus;

#[async_trait]
impl EventBus for LocalEventBus {
    async fn publish(&self, topic: &str, payload: &str) -> anyhow::Result<()> {
        println!("[event][{}] {}", topic, payload);
        Ok(())
    }
}

// Hàm khởi tạo
pub fn get_event_bus() -> impl EventBus {
    LocalEventBus
}
