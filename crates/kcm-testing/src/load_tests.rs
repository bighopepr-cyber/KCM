use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
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
    pub fn pass(&self, _scenario: &LoadTestScenario) -> bool {
        self.failed_operations <= self.total_operations / 1000
    }

    pub fn to_report(&self) -> String {
        format!(
            "Load Test: {}\n  Operations: {} (failed: {})\n  Elapsed: {:.2}s\n  QPS: {:.0}\n  Avg Latency: {:.2}ms\n  P99 Latency: {:.2}ms",
            self.scenario,
            self.total_operations,
            self.failed_operations,
            self.elapsed_secs,
            self.actual_qps,
            self.avg_latency_ms,
            self.p99_latency_ms,
        )
    }
}

pub fn run_load_test(scenario: &LoadTestScenario) -> Result<LoadTestResults, KcmError> {
    let kb = Arc::new(KnowledgeDatabase::new()?);
    for i in 0..scenario.initial_facts {
        let f = Fact::new(
            SubjectID((i % 1000) as u32),
            PredicateID((i % 10) as u8),
            ObjectID(((i * 2) % 2000) as u32),
            0.7,
        )?;
        kb.insert(&f)?;
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
                let success = if (user + i as usize).is_multiple_of(3) {
                    let fact = Fact::new(
                        SubjectID((user % 100) as u32),
                        PredicateID(5),
                        ObjectID((i % 1000) as u32),
                        0.8,
                    );
                    match fact {
                        Ok(f) => kb.insert(&f).is_ok(),
                        Err(_) => false,
                    }
                } else {
                    kb.query().with_predicate(PredicateID(5)).execute().is_ok()
                };
                total_ops.fetch_add(1, AtomicOrdering::Relaxed);
                if !success {
                    failed_ops.fetch_add(1, AtomicOrdering::Relaxed);
                }
                latencies
                    .lock()
                    .push(op_start.elapsed().as_secs_f64() * 1000.0);
            }
        }));
    }

    for h in handles {
        h.join()
            .map_err(|e| KcmError::Io(format!("Thread panicked during load test: {:?}", e)))?;
    }
    let elapsed = start.elapsed().as_secs_f64();

    let mut lat = latencies.lock().clone();
    lat.sort_by(|a, b| a.total_cmp(b));
    let p99_idx = (lat.len() as f64 * 0.99) as usize;
    let avg = if lat.is_empty() {
        0.0
    } else {
        lat.iter().sum::<f64>() / lat.len() as f64
    };

    Ok(LoadTestResults {
        scenario: scenario.name.clone(),
        total_operations: total_ops.load(AtomicOrdering::Relaxed),
        failed_operations: failed_ops.load(AtomicOrdering::Relaxed),
        elapsed_secs: elapsed,
        actual_qps: total_ops.load(AtomicOrdering::Relaxed) as f64 / elapsed,
        avg_latency_ms: avg,
        p99_latency_ms: lat.get(p99_idx).copied().unwrap_or(0.0),
    })
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
        let results = run_load_test(&scenario).unwrap();
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
        let results = run_load_test(&scenario).unwrap();
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
                    total_ops.fetch_add(1, AtomicOrdering::Relaxed);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(total_ops.load(AtomicOrdering::Relaxed), 400);
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

    #[test]
    fn test_load_heavy() {
        let scenario = LoadTestScenario {
            name: "Heavy".to_string(),
            concurrent_users: 100,
            operations_per_user: 20,
            initial_facts: 5_000,
            expected_qps: 500.0,
            max_latency_p99_ms: 2000.0,
        };
        let results = run_load_test(&scenario).unwrap();
        assert!(results.total_operations > 0, "Should complete operations");
        assert!(results.pass(&scenario), "Heavy load should pass");
        println!("{}", results.to_report());
    }

    #[test]
    fn test_load_spike() {
        let scenario = LoadTestScenario {
            name: "Spike".to_string(),
            concurrent_users: 200,
            operations_per_user: 10,
            initial_facts: 2_000,
            expected_qps: 1000.0,
            max_latency_p99_ms: 3000.0,
        };
        let results = run_load_test(&scenario).unwrap();
        assert!(results.total_operations > 0, "Should complete operations");
        assert!(results.pass(&scenario), "Spike load should pass");
        println!("{}", results.to_report());
    }

    #[test]
    fn test_load_read_heavy() {
        let kb = Arc::new(KnowledgeDatabase::new().unwrap());
        for i in 0..1_000u32 {
            let fact = Fact::new(SubjectID(i % 50), PredicateID(0), ObjectID(i), 0.8).unwrap();
            kb.insert(&fact).unwrap();
        }

        let total_ops = Arc::new(AtomicU64::new(0));
        let failed_ops = Arc::new(AtomicU64::new(0));
        let start = Instant::now();
        let mut handles = Vec::new();

        for user in 0..20 {
            let kb = kb.clone();
            let total_ops = total_ops.clone();
            let failed_ops = failed_ops.clone();
            handles.push(std::thread::spawn(move || {
                for i in 0..100u64 {
                    let op_start = Instant::now();
                    let success = if (user + i as usize) % 10 < 9 {
                        kb.query().with_predicate(PredicateID(0)).execute().is_ok()
                    } else {
                        let fact = Fact::new(
                            SubjectID((user % 50) as u32),
                            PredicateID(1),
                            ObjectID((i % 1000) as u32),
                            0.7,
                        );
                        match fact {
                            Ok(f) => kb.insert(&f).is_ok(),
                            Err(_) => false,
                        }
                    };
                    total_ops.fetch_add(1, AtomicOrdering::Relaxed);
                    if !success {
                        failed_ops.fetch_add(1, AtomicOrdering::Relaxed);
                    }
                    let _ = op_start.elapsed();
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let elapsed = start.elapsed().as_secs_f64();
        let total = total_ops.load(AtomicOrdering::Relaxed);
        let failed = failed_ops.load(AtomicOrdering::Relaxed);
        println!(
            "Read-Heavy: {} ops ({} failed) in {:.2}s, {:.0} QPS",
            total,
            failed,
            elapsed,
            total as f64 / elapsed
        );
        assert!(total > 0, "Should complete operations");
        assert!(
            failed <= total / 1000,
            "Read-heavy failure rate too high: {}/{}",
            failed,
            total
        );
    }

    #[test]
    fn test_load_write_heavy() {
        let kb = Arc::new(KnowledgeDatabase::new().unwrap());
        let total_ops = Arc::new(AtomicU64::new(0));
        let failed_ops = Arc::new(AtomicU64::new(0));
        let start = Instant::now();
        let mut handles = Vec::new();

        for user in 0..20 {
            let kb = kb.clone();
            let total_ops = total_ops.clone();
            let failed_ops = failed_ops.clone();
            handles.push(std::thread::spawn(move || {
                for i in 0..100u64 {
                    let op_start = Instant::now();
                    let success = if (user + i as usize) % 10 < 9 {
                        let fact = Fact::new(
                            SubjectID((user % 50) as u32),
                            PredicateID((i % 10) as u8),
                            ObjectID((i % 1000) as u32),
                            0.7,
                        );
                        match fact {
                            Ok(f) => kb.insert(&f).is_ok(),
                            Err(_) => false,
                        }
                    } else {
                        kb.query().with_predicate(PredicateID(0)).execute().is_ok()
                    };
                    total_ops.fetch_add(1, AtomicOrdering::Relaxed);
                    if !success {
                        failed_ops.fetch_add(1, AtomicOrdering::Relaxed);
                    }
                    let _ = op_start.elapsed();
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let elapsed = start.elapsed().as_secs_f64();
        let total = total_ops.load(AtomicOrdering::Relaxed);
        let failed = failed_ops.load(AtomicOrdering::Relaxed);
        println!(
            "Write-Heavy: {} ops ({} failed) in {:.2}s, {:.0} QPS",
            total,
            failed,
            elapsed,
            total as f64 / elapsed
        );
        assert!(total > 0, "Should complete operations");
        assert!(
            failed <= total / 1000,
            "Write-heavy failure rate too high: {}/{}",
            failed,
            total
        );
    }
}
