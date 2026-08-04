use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Token bucket rate limiter per client IP.
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, TokenBucket>>,
    max_requests: u32,
    window: Duration,
}

struct TokenBucket {
    tokens: u32,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        RateLimiter {
            buckets: Mutex::new(HashMap::new()),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn allow(&self, client_id: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();
        let bucket = buckets
            .entry(client_id.to_string())
            .or_insert_with(|| TokenBucket {
                tokens: self.max_requests,
                last_refill: now,
            });

        let elapsed = now.duration_since(bucket.last_refill);
        if elapsed >= self.window {
            bucket.tokens = self.max_requests;
            bucket.last_refill = now;
        }

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }

    pub fn max_requests(&self) -> u32 {
        self.max_requests
    }
}
