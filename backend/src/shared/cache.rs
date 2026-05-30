use moka::sync::Cache;
use serde_json::Value;
use std::time::Duration;

/// In-memory cache for API responses.
///
/// Cache TTLs are tuned per endpoint:
/// - stats: 30s  (aggregate queries, slow)
/// - config: 60s (rarely changes)
/// - sessions: 10s (changes more often)
#[derive(Clone)]
pub struct ApiCache {
    pub stats: Cache<String, Value>,
    pub config: Cache<String, Value>,
    pub sessions: Cache<String, Value>,
}

impl ApiCache {
    pub fn new() -> Self {
        Self {
            stats: Cache::builder()
                .max_capacity(10)
                .time_to_live(Duration::from_secs(30))
                .build(),
            config: Cache::builder()
                .max_capacity(5)
                .time_to_live(Duration::from_secs(60))
                .build(),
            sessions: Cache::builder()
                .max_capacity(20)
                .time_to_live(Duration::from_secs(10))
                .build(),
        }
    }

    /// Invalidate all caches (called after write operations).
    pub fn invalidate_all(&self) {
        self.stats.invalidate_all();
        self.config.invalidate_all();
        self.sessions.invalidate_all();
    }
}

/// Try to get from cache; if miss, return None so handler can fetch.
pub fn get_cached(cache: &Cache<String, Value>, key: &str) -> Option<Value> {
    let val = cache.get(&key.to_string());
    if val.is_some() {
        tracing::debug!("Cache HIT for key: {}", key);
    } else {
        tracing::debug!("Cache MISS for key: {}", key);
    }
    val
}

/// Insert a value into cache.
pub fn insert_cached(cache: &Cache<String, Value>, key: &str, value: Value) {
    cache.insert(key.to_string(), value);
}
