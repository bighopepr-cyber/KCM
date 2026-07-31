use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::executor::Executor;
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
        self.actual_qps >= scenario.expected_qps * 0.95
            && self.p99_latency_ms <= scenario.max_latency_p99_ms
            && self.failed_operations <= self.total_operations / 1000
    }
}

pub fn light_scenario() -> LoadTestScenario {
    LoadTestScenario {
        name: "Light".to_string(),
        concurrent_users: 10,
        operations_per_user: 100,
        initial_facts: 10_000,
        expected_qps: 5_000.0,
        max_latency_p99_ms: 10.0,
    }
}

pub fn medium_scenario() -> LoadTestScenario {
    LoadTestScenario {
        name: "Medium".to_string(),
        concurrent_users: 50,
        operations_per_user: 200,
        initial_facts: 100_000,
        expected_qps: 15_000.0,
        max_latency_p99_ms: 20.0,
    }
}

pub fn heavy_scenario() -> LoadTestScenario {
    LoadTestScenario {
        name: "Heavy".to_string(),
        concurrent_users: 100,
        operations_per_user: 500,
        initial_facts: 1_000_000,
        expected_qps: 25_000.0,
        max_latency_p99_ms: 50.0,
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
    let executor = Executor::new(scenario.concurrent_users.min(16)).unwrap();

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
    let avg = lat.iter().sum::<f64>() / lat.len() as f64;

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
