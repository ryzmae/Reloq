mod rate_limit;
mod storage;

use rate_limit::RateLimit;
use std::sync::Arc;
use storage::Storage;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let storage = Arc::new(Mutex::new(Storage::new("reloq.db").unwrap()));
    let rate_limiter = RateLimit::new(storage.clone(), 5); // Limit: 5 requests

    let user_id = "user123";

    if rate_limiter.check_limit(user_id).await {
        rate_limiter.update_limit(user_id).await;
        println!("✅ Request allowed for {}", user_id);
    } else {
        println!("⛔ Rate limit exceeded for {}", user_id);
    }
}
