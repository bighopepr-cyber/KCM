use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct StressTestScenario {
    pub name: String,
    pub max_concurrent_users: usize,
    pub duration_secs: u64,
}

pub struct StressTestResults {
    pub scenario: String,
    pub total_ops: u64,
    pub failed_ops: u64,
    pub elapsed_secs: f64,
    pub peak_qps: f64,
    pub failure_rate: f64,
    pub graceful_degradation: bool,
}

impl StressTestResults {
    pub fn to_report(&self) -> String {
        format!(
            "Stress Test: {}\n  Operations: {} (failed: {})\n  Elapsed: {:.2}s\n  Peak QPS: {:.0}\n  Failure Rate: {:.4}%\n  Graceful Degradation: {}",
            self.scenario, self.total_ops, self.failed_ops,
            self.elapsed_secs, self.peak_qps, self.failure_rate * 100.0,
            self.graceful_degradation,
        )
    }
}

pub struct StressTestConfig {
    pub name: String,
    pub max_concurrent_users: usize,
    pub ops_per_user: u64,
    pub batch_size: usize,
}

pub struct StressTestConfigResults {
    pub scenario: String,
    pub total_ops: u64,
    pub failed_ops: u64,
    pub elapsed_secs: f64,
    pub actual_qps: f64,
    pub failure_rate: f64,
    pub graceful_degradation: bool,
}

impl StressTestConfigResults {
    pub fn to_report(&self) -> String {
        format!(
            "Stress Config Test: {}\n  Operations: {} (failed: {})\n  Elapsed: {:.2}s\n  QPS: {:.0}\n  Failure Rate: {:.4}%\n  Graceful Degradation: {}",
            self.scenario, self.total_ops, self.failed_ops,
            self.elapsed_secs, self.actual_qps, self.failure_rate * 100.0,
            self.graceful_degradation,
        )
    }
}

pub fn run_stress_test_config(
    config: &StressTestConfig,
) -> Result<StressTestConfigResults, KcmError> {
    let kb = Arc::new(KnowledgeDatabase::new()?);
    let total_ops = Arc::new(AtomicU64::new(0));
    let failed_ops = Arc::new(AtomicU64::new(0));

    let start = Instant::now();
    let mut handles = Vec::new();

    for user in 0..config.max_concurrent_users {
        let kb = kb.clone();
        let total_ops = total_ops.clone();
        let failed_ops = failed_ops.clone();
        let ops = config.ops_per_user;
        let batch = config.batch_size;

        handles.push(std::thread::spawn(move || {
            for i in 0..ops {
                let success = if i % (batch as u64 + 1) == 0 {
                    let fact = Fact::new(
                        SubjectID((user % 100) as u32),
                        PredicateID((i % 10) as u8),
                        ObjectID((i % 1000) as u32),
                        0.7,
                    );
                    match fact {
                        Ok(f) => kb.insert(&f).is_ok(),
                        Err(_) => false,
                    }
                } else {
                    kb.query().with_predicate(PredicateID(5)).execute().is_ok()
                };
                total_ops.fetch_add(1, Ordering::Relaxed);
                if !success {
                    failed_ops.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }

    for h in handles {
        h.join()
            .map_err(|e| KcmError::Io(format!("Thread panicked during stress test: {:?}", e)))?;
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total = total_ops.load(Ordering::Relaxed);
    let failed = failed_ops.load(Ordering::Relaxed);

    Ok(StressTestConfigResults {
        scenario: config.name.clone(),
        total_ops: total,
        failed_ops: failed,
        elapsed_secs: elapsed,
        actual_qps: total as f64 / elapsed,
        failure_rate: if total > 0 {
            failed as f64 / total as f64
        } else {
            0.0
        },
        graceful_degradation: (failed as f64 / total.max(1) as f64) < 0.10,
    })
}

pub fn run_stress_test(scenario: &StressTestScenario) -> Result<StressTestResults, KcmError> {
    let kb = Arc::new(KnowledgeDatabase::new()?);
    let running = Arc::new(AtomicBool::new(true));
    let total_ops = Arc::new(AtomicU64::new(0));
    let failed_ops = Arc::new(AtomicU64::new(0));

    let start = Instant::now();
    let mut handles = Vec::new();

    for user in 0..scenario.max_concurrent_users {
        let kb = kb.clone();
        let running = running.clone();
        let total_ops = total_ops.clone();
        let failed_ops = failed_ops.clone();

        handles.push(std::thread::spawn(move || {
            let mut count = 0u64;
            while running.load(Ordering::Relaxed) {
                let fact = Fact::new(
                    SubjectID((user % 100) as u32),
                    PredicateID((count % 10) as u8),
                    ObjectID((count % 1000) as u32),
                    (0.5 + (count as f64 % 0.5)).min(0.99),
                );

                let success = match fact {
                    Ok(f) => {
                        if count.is_multiple_of(3) {
                            kb.insert(&f).is_ok()
                        } else {
                            kb.query().with_predicate(PredicateID(5)).execute().is_ok()
                        }
                    }
                    Err(_) => false,
                };

                total_ops.fetch_add(1, Ordering::Relaxed);
                if !success {
                    failed_ops.fetch_add(1, Ordering::Relaxed);
                }
                count += 1;
            }
        }));
    }

    std::thread::sleep(Duration::from_secs(scenario.duration_secs));
    running.store(false, Ordering::Relaxed);
    for h in handles {
        h.join()
            .map_err(|e| KcmError::Io(format!("Thread panicked during stress test: {:?}", e)))?;
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total = total_ops.load(Ordering::Relaxed);
    let failed = failed_ops.load(Ordering::Relaxed);

    Ok(StressTestResults {
        scenario: scenario.name.clone(),
        total_ops: total,
        failed_ops: failed,
        elapsed_secs: elapsed,
        peak_qps: total as f64 / elapsed,
        failure_rate: if total > 0 {
            failed as f64 / total as f64
        } else {
            0.0
        },
        graceful_degradation: (failed as f64 / total.max(1) as f64) < 0.10,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_sustained_load() {
        let scenario = StressTestScenario {
            name: "Sustained Load".to_string(),
            max_concurrent_users: 8,
            duration_secs: 2,
        };
        let results = run_stress_test(&scenario).unwrap();
        assert!(results.total_ops > 0, "Should complete operations");
        assert!(results.graceful_degradation, "Should degrade gracefully");
        println!("{}", results.to_report());
    }

    #[test]
    fn test_stress_spike() {
        let scenario = StressTestScenario {
            name: "Spike".to_string(),
            max_concurrent_users: 16,
            duration_secs: 1,
        };
        let results = run_stress_test(&scenario).unwrap();
        assert!(results.total_ops > 0);
        println!("{}", results.to_report());
    }

    #[test]
    fn test_stress_zero_users() {
        let scenario = StressTestScenario {
            name: "No Users".to_string(),
            max_concurrent_users: 0,
            duration_secs: 1,
        };
        let results = run_stress_test(&scenario).unwrap();
        assert_eq!(results.total_ops, 0);
        assert!(results.graceful_degradation);
    }

    #[test]
    fn test_stress_memory_exhaustion() {
        let config = StressTestConfig {
            name: "Memory Exhaustion".to_string(),
            max_concurrent_users: 16,
            ops_per_user: 500,
            batch_size: 50,
        };
        let results = run_stress_test_config(&config).unwrap();
        assert!(results.total_ops > 0, "Should complete operations");
        assert!(
            results.graceful_degradation,
            "Should degrade gracefully under memory pressure"
        );
        println!("{}", results.to_report());
    }

    #[test]
    fn test_stress_memory_exhaustion_large_batch() {
        let config = StressTestConfig {
            name: "Memory Exhaustion Large Batch".to_string(),
            max_concurrent_users: 32,
            ops_per_user: 200,
            batch_size: 200,
        };
        let results = run_stress_test_config(&config).unwrap();
        assert!(results.total_ops > 0, "Should complete operations");
        println!("{}", results.to_report());
    }
}
