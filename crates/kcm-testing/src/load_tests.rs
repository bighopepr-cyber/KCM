use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub struct LoadTestScenario {
    pub name: String,
    pub concurrent_users: usize,
    pub operations_per_user: u64,
    pub initial_facts: u64,
    pub expected_qps: f64,
    pub max_latency_p99_ms: f64,
}

pub struct LoadTestResults {
    pub scenario: String,
    pub total_operations: u64,
    pub failed_operations: u64,
    pub elapsed_secs: f64,
    pub actual_qps: f64,
    pub avg_latency_ms: f64,
    pub p99_latency_ms: f64,
}

impl LoadTestResults {
    pub fn pass(&self, scenario: &LoadTestScenario) -> bool {
        self.failed_operations <= self.total_operations / 1000
    }

    pub fn to_report(&self) -> String {
        format!(
            "Load Test: {}\n  Operations: {} (failed: {})\n  Elapsed: {:.2}s\n  QPS: {:.0}\n  Avg Latency: {:.2}ms\n  P99 Latency: {:.2}ms",
            self.scenario, self.total_operations, self.failed_operations,
            self.elapsed_secs, self.actual_qps, self.avg_latency_ms, self.p99_latency_ms,
        )
    }
}

pub fn run_load_test(scenario: &LoadTestScenario) -> LoadTestResults {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    for i in 0..scenario.initial_facts {
        let f = Fact::new(
            SubjectID((i % 1000) as u32),
            PredicateID((i % 10) as u8),
            ObjectID(((i * 2) % 2000) as u32),
            0.7,
        )
        .unwrap();
        kb.insert(&f).unwrap();
    }

    let total_ops = Arc::new(AtomicU64::new(0));
    let failed_ops = Arc::new(AtomicU64::new(0));
    let latencies = Arc::new(parking_lot::Mutex::new(Vec::<f64>::new()));

    let start = Instant::now();
    let mut handles = Vec::new();

    for user in 0..scenario.concurrent_users {
        let kb = kb.clone();
        let total_ops = total_ops.clone();
        let failed_ops = failed_ops.clone();
        let latencies = latencies.clone();
        let ops = scenario.operations_per_user;

        handles.push(std::thread::spawn(move || {
            for i in 0..ops {
                let op_start = Instant::now();
                let success = if (user + i as usize) % 3 == 0 {
                    let fact = Fact::new(
                        SubjectID((user % 100) as u32),
                        PredicateID(5),
                        ObjectID((i % 1000) as u32),
                        0.8,
                    )
                    .unwrap();
                    kb.insert(&fact).is_ok()
                } else {
                    kb.query().with_predicate(PredicateID(5)).execute().is_ok()
                };
                total_ops.fetch_add(1, Ordering::Relaxed);
                if !success {
                    failed_ops.fetch_add(1, Ordering::Relaxed);
                }
                latencies
                    .lock()
                    .push(op_start.elapsed().as_secs_f64() * 1000.0);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    let elapsed = start.elapsed().as_secs_f64();

    let mut lat = latencies.lock().clone();
    lat.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p99_idx = (lat.len() as f64 * 0.99) as usize;
    let avg = if lat.is_empty() {
        0.0
    } else {
        lat.iter().sum::<f64>() / lat.len() as f64
    };

    LoadTestResults {
        scenario: scenario.name.clone(),
        total_operations: total_ops.load(Ordering::Relaxed),
        failed_operations: failed_ops.load(Ordering::Relaxed),
        elapsed_secs: elapsed,
        actual_qps: total_ops.load(Ordering::Relaxed) as f64 / elapsed,
        avg_latency_ms: avg,
        p99_latency_ms: lat.get(p99_idx).copied().unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_light() {
        let scenario = LoadTestScenario {
            name: "Light".to_string(),
            concurrent_users: 4,
            operations_per_user: 50,
            initial_facts: 100,
            expected_qps: 100.0,
            max_latency_p99_ms: 500.0,
        };
        let results = run_load_test(&scenario);
        assert!(results.total_operations > 0, "Should complete operations");
        assert_eq!(results.failed_operations, 0, "No operations should fail");
        assert!(results.pass(&scenario), "Light load should pass");
        println!("{}", results.to_report());
    }

    #[test]
    fn test_load_medium() {
        let scenario = LoadTestScenario {
            name: "Medium".to_string(),
            concurrent_users: 8,
            operations_per_user: 100,
            initial_facts: 1_000,
            expected_qps: 100.0,
            max_latency_p99_ms: 1000.0,
        };
        let results = run_load_test(&scenario);
        assert!(results.total_operations > 0);
        assert!(results.pass(&scenario));
        println!("{}", results.to_report());
    }

    #[test]
    fn test_load_concurrent_inserts() {
        let kb = Arc::new(KnowledgeDatabase::new().unwrap());
        let total_ops = Arc::new(AtomicU64::new(0));

        let mut handles = Vec::new();
        for t in 0..4u32 {
            let kb = kb.clone();
            let total_ops = total_ops.clone();
            handles.push(std::thread::spawn(move || {
                for i in 0..100u32 {
                    let fact = Fact::new(SubjectID(t * 1000 + i), PredicateID(0), ObjectID(i), 0.9)
                        .unwrap();
                    kb.insert(&fact).unwrap();
                    total_ops.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(total_ops.load(Ordering::Relaxed), 400);
        assert_eq!(kb.fact_count(), 400);
    }

    #[test]
    fn test_load_concurrent_queries() {
        let kb = Arc::new(KnowledgeDatabase::new().unwrap());
        for i in 0..200u32 {
            let fact = Fact::new(SubjectID(i % 10), PredicateID(0), ObjectID(i), 0.8).unwrap();
            kb.insert(&fact).unwrap();
        }

        let mut handles = Vec::new();
        for _ in 0..4 {
            let kb = kb.clone();
            handles.push(std::thread::spawn(move || {
                for _ in 0..50 {
                    let results = kb.query().with_predicate(PredicateID(0)).execute().unwrap();
                    assert!(!results.is_empty());
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
