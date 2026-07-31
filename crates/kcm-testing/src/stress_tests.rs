use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct StressTestScenario {
    pub name: String,
    pub max_concurrent_users: usize,
    pub duration_secs: u64,
    pub memory_limit_mb: u64,
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

pub fn sustained_load_scenario() -> StressTestScenario {
    StressTestScenario {
        name: "Sustained Load".to_string(),
        max_concurrent_users: 50,
        duration_secs: 10,
        memory_limit_mb: 4096,
    }
}

pub fn spike_scenario() -> StressTestScenario {
    StressTestScenario {
        name: "Spike Load".to_string(),
        max_concurrent_users: 200,
        duration_secs: 5,
        memory_limit_mb: 4096,
    }
}

pub fn run_stress_test(scenario: &StressTestScenario) -> StressTestResults {
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
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
                    0.5 + (count as f64 % 0.5).min(1.0),
                )
                .unwrap();

                let success = if count % 3 == 0 {
                    kb.insert(&fact).is_ok()
                } else {
                    kb.query().with_predicate(PredicateID(5)).execute().is_ok()
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
        h.join().unwrap();
    }
    let elapsed = start.elapsed().as_secs_f64();
    let total = total_ops.load(Ordering::Relaxed);
    let failed = failed_ops.load(Ordering::Relaxed);

    StressTestResults {
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
    }
}
