use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type SharedPriceStore = Arc<Mutex<HashMap<String, String>>>;

pub fn new_price_store() -> SharedPriceStore {
    Arc::new(Mutex::new(HashMap::new()))
}
