use crate::storage::Storage;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RateLimit {
    storage: Arc<Mutex<Storage>>,
    max_requests: u64,
}

impl RateLimit {
    pub fn new(storage: Arc<Mutex<Storage>>, max_requests: u64) -> Self {
        RateLimit {
            storage,
            max_requests,
        }
    }

    /// Checks if a user has reached their rate limit.
    pub async fn check_limit(&self, user_id: &str) -> bool {
        let storage = self.storage.lock().await;

        if let Ok(Some(current_usage)) = storage.get_rate_limit(user_id) {
            return current_usage < self.max_requests;
        }
        true
    }

    pub async fn update_limit(&self, user_id: &str) {
        let storage = self.storage.lock().await;

        if let Ok(Some(current_usage)) = storage.get_rate_limit(user_id) {
            let new_usage = current_usage + 1;
            let _ = storage.set_rate_limit(user_id, new_usage);
        } else {
            let _ = storage.set_rate_limit(user_id, 1);
        }
    }
}
