use crate::error::AppError;
use std::{
    collections::{HashMap, VecDeque},
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing::warn;

#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<HashMap<IpAddr, VecDeque<Instant>>>>,
    limit: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(per_minute: u32, burst: u32) -> Self {
        let limit = burst.max(per_minute).max(1);
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            limit,
            window: Duration::from_secs(60),
        }
    }

    pub async fn check(&self, ip: IpAddr) -> Result<(), AppError> {
        let mut guard = self.inner.lock().await;
        let deque = guard.entry(ip).or_insert_with(VecDeque::new);
        let now = Instant::now();
        let cutoff = now - self.window;

        while matches!(deque.front(), Some(&ts) if ts < cutoff) {
            deque.pop_front();
        }

        if deque.len() as u32 >= self.limit {
            warn!(%ip, "rate limit exceeded for client");
            return Err(AppError::TooManyRequests);
        }

        deque.push_back(now);
        Ok(())
    }
}

