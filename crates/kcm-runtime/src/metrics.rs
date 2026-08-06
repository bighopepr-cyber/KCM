use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Internal metrics counters. Allocated once behind a single Arc.
/// All 13 counters occupy 104 bytes contiguous.
pub struct MetricsInner {
    pub queries_total: AtomicU64,
    pub queries_failed: AtomicU64,
    pub query_duration_sum_ms: AtomicU64,
    pub inserts_total: AtomicU64,
    pub inserts_failed: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub inferences_total: AtomicU64,
    pub facts_inferred: AtomicU64,
    pub estimated_memory_bytes: AtomicU64,
    pub total_facts: AtomicU64,
    pub active_facts: AtomicU64,
    pub tombstone_count: AtomicU64,
}

pub struct Metrics {
    inner: Arc<MetricsInner>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            inner: Arc::new(MetricsInner {
                queries_total: AtomicU64::new(0),
                queries_failed: AtomicU64::new(0),
                query_duration_sum_ms: AtomicU64::new(0),
                inserts_total: AtomicU64::new(0),
                inserts_failed: AtomicU64::new(0),
                cache_hits: AtomicU64::new(0),
                cache_misses: AtomicU64::new(0),
                inferences_total: AtomicU64::new(0),
                facts_inferred: AtomicU64::new(0),
                estimated_memory_bytes: AtomicU64::new(0),
                total_facts: AtomicU64::new(0),
                active_facts: AtomicU64::new(0),
                tombstone_count: AtomicU64::new(0),
            }),
        }
    }

    pub fn record_query(&self, duration_ms: u64, success: bool) {
        self.inner.queries_total.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.inner.queries_failed.fetch_add(1, Ordering::Relaxed);
        }
        self.inner
            .query_duration_sum_ms
            .fetch_add(duration_ms, Ordering::Relaxed);
    }

    pub fn record_insert(&self, success: bool) {
        self.inner.inserts_total.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.inner.inserts_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_cache_hit(&self) {
        self.inner.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.inner.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_inference(&self, facts_derived: u64) {
        self.inner.inferences_total.fetch_add(1, Ordering::Relaxed);
        self.inner
            .facts_inferred
            .fetch_add(facts_derived, Ordering::Relaxed);
    }

    pub fn get_avg_query_latency_ms(&self) -> f64 {
        let total = self.inner.queries_total.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        self.inner.query_duration_sum_ms.load(Ordering::Relaxed) as f64 / total as f64
    }

    pub fn get_cache_hit_ratio(&self) -> f64 {
        let hits = self.inner.cache_hits.load(Ordering::Relaxed);
        let misses = self.inner.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    pub fn get_insert_error_rate(&self) -> f64 {
        let total = self.inner.inserts_total.load(Ordering::Relaxed);
        if total == 0 {
            0.0
        } else {
            self.inner.inserts_failed.load(Ordering::Relaxed) as f64 / total as f64
        }
    }

    pub fn get_query_error_rate(&self) -> f64 {
        let total = self.inner.queries_total.load(Ordering::Relaxed);
        if total == 0 {
            0.0
        } else {
            self.inner.queries_failed.load(Ordering::Relaxed) as f64 / total as f64
        }
    }

    pub fn update_memory_estimate(&self, bytes: u64) {
        self.inner
            .estimated_memory_bytes
            .store(bytes, Ordering::Relaxed);
    }

    pub fn update_schema_stats(&self, total: u64, active: u64, tombstones: u64) {
        self.inner.total_facts.store(total, Ordering::Relaxed);
        self.inner.active_facts.store(active, Ordering::Relaxed);
        self.inner
            .tombstone_count
            .store(tombstones, Ordering::Relaxed);
    }

    pub fn estimated_memory_mb(&self) -> f64 {
        self.inner.estimated_memory_bytes.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0)
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            queries_total: self.inner.queries_total.load(Ordering::Relaxed),
            queries_failed: self.inner.queries_failed.load(Ordering::Relaxed),
            avg_query_latency_ms: self.get_avg_query_latency_ms(),
            inserts_total: self.inner.inserts_total.load(Ordering::Relaxed),
            inserts_failed: self.inner.inserts_failed.load(Ordering::Relaxed),
            cache_hit_ratio: self.get_cache_hit_ratio(),
            inferences_total: self.inner.inferences_total.load(Ordering::Relaxed),
            facts_inferred: self.inner.facts_inferred.load(Ordering::Relaxed),
            estimated_memory_bytes: self.inner.estimated_memory_bytes.load(Ordering::Relaxed),
            total_facts: self.inner.total_facts.load(Ordering::Relaxed),
            active_facts: self.inner.active_facts.load(Ordering::Relaxed),
            tombstone_count: self.inner.tombstone_count.load(Ordering::Relaxed),
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
    pub inferences_total: u64,
    pub facts_inferred: u64,
    pub estimated_memory_bytes: u64,
    pub total_facts: u64,
    pub active_facts: u64,
    pub tombstone_count: u64,
}

impl MetricsSnapshot {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"queries_total":{},"queries_failed":{},"avg_query_latency_ms":{:.2},"inserts_total":{},"inserts_failed":{},"cache_hit_ratio":{:.4},"inferences_total":{},"facts_inferred":{},"estimated_memory_bytes":{},"total_facts":{},"active_facts":{},"tombstone_count":{}}}"#,
            self.queries_total,
            self.queries_failed,
            self.avg_query_latency_ms,
            self.inserts_total,
            self.inserts_failed,
            self.cache_hit_ratio,
            self.inferences_total,
            self.facts_inferred,
            self.estimated_memory_bytes,
            self.total_facts,
            self.active_facts,
            self.tombstone_count,
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
