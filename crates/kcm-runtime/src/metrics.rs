use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub struct Metrics {
    pub queries_total: Arc<AtomicU64>,
    pub queries_failed: Arc<AtomicU64>,
    pub query_duration_sum_ms: Arc<AtomicU64>,
    pub inserts_total: Arc<AtomicU64>,
    pub inserts_failed: Arc<AtomicU64>,
    pub cache_hits: Arc<AtomicU64>,
    pub cache_misses: Arc<AtomicU64>,
    pub memory_bytes: Arc<AtomicU64>,
    pub column_count: Arc<AtomicU64>,
    pub inferences_total: Arc<AtomicU64>,
    pub facts_inferred: Arc<AtomicU64>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            queries_total: Arc::new(AtomicU64::new(0)),
            queries_failed: Arc::new(AtomicU64::new(0)),
            query_duration_sum_ms: Arc::new(AtomicU64::new(0)),
            inserts_total: Arc::new(AtomicU64::new(0)),
            inserts_failed: Arc::new(AtomicU64::new(0)),
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
            memory_bytes: Arc::new(AtomicU64::new(0)),
            column_count: Arc::new(AtomicU64::new(0)),
            inferences_total: Arc::new(AtomicU64::new(0)),
            facts_inferred: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_query(&self, duration_ms: u64, success: bool) {
        self.queries_total.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.queries_failed.fetch_add(1, Ordering::Relaxed);
        }
        self.query_duration_sum_ms
            .fetch_add(duration_ms, Ordering::Relaxed);
    }

    pub fn record_insert(&self, success: bool) {
        self.inserts_total.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.inserts_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_avg_query_latency_ms(&self) -> f64 {
        let total = self.queries_total.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        self.query_duration_sum_ms.load(Ordering::Relaxed) as f64 / total as f64
    }

    pub fn get_cache_hit_ratio(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    pub fn get_insert_error_rate(&self) -> f64 {
        let total = self.inserts_total.load(Ordering::Relaxed);
        if total == 0 {
            0.0
        } else {
            self.inserts_failed.load(Ordering::Relaxed) as f64 / total as f64
        }
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            queries_total: self.queries_total.load(Ordering::Relaxed),
            queries_failed: self.queries_failed.load(Ordering::Relaxed),
            avg_query_latency_ms: self.get_avg_query_latency_ms(),
            inserts_total: self.inserts_total.load(Ordering::Relaxed),
            inserts_failed: self.inserts_failed.load(Ordering::Relaxed),
            cache_hit_ratio: self.get_cache_hit_ratio(),
            memory_bytes: self.memory_bytes.load(Ordering::Relaxed),
            inferences_total: self.inferences_total.load(Ordering::Relaxed),
            facts_inferred: self.facts_inferred.load(Ordering::Relaxed),
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub queries_total: u64,
    pub queries_failed: u64,
    pub avg_query_latency_ms: f64,
    pub inserts_total: u64,
    pub inserts_failed: u64,
    pub cache_hit_ratio: f64,
    pub memory_bytes: u64,
    pub inferences_total: u64,
    pub facts_inferred: u64,
}

impl MetricsSnapshot {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"queries_total":{},"queries_failed":{},"avg_query_latency_ms":{:.2},"inserts_total":{},"inserts_failed":{},"cache_hit_ratio":{:.4},"memory_bytes":{},"inferences_total":{},"facts_inferred":{}}}"#,
            self.queries_total,
            self.queries_failed,
            self.avg_query_latency_ms,
            self.inserts_total,
            self.inserts_failed,
            self.cache_hit_ratio,
            self.memory_bytes,
            self.inferences_total,
            self.facts_inferred,
        )
    }
}

pub struct ScopedTimer {
    start: Instant,
    duration_ms: Arc<AtomicU64>,
}

impl ScopedTimer {
    pub fn new(duration_ms: Arc<AtomicU64>) -> Self {
        ScopedTimer {
            start: Instant::now(),
            duration_ms,
        }
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_millis() as u64;
        self.duration_ms.fetch_add(elapsed, Ordering::Relaxed);
    }
}
