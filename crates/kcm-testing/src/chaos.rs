use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct ChaosConfig {
    pub packet_loss_percent: f64,
    pub latency_ms: u64,
    pub partition_duration: Duration,
    pub node_failure_count: usize,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        ChaosConfig {
            packet_loss_percent: 0.0,
            latency_ms: 0,
            partition_duration: Duration::from_secs(5),
            node_failure_count: 0,
        }
    }
}

pub struct ChaosMonkey {
    active: Arc<AtomicBool>,
    failure_count: Arc<AtomicU64>,
    config: ChaosConfig,
}

impl ChaosMonkey {
    pub fn new(config: ChaosConfig) -> Self {
        ChaosMonkey {
            active: Arc::new(AtomicBool::new(false)),
            failure_count: Arc::new(AtomicU64::new(0)),
            config,
        }
    }

    pub fn activate(&self) {
        self.active.store(true, Ordering::SeqCst);
    }

    pub fn deactivate(&self) {
        self.active.store(false, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    pub fn should_inject_failure(&self) -> bool {
        if !self.is_active() {
            return false;
        }
        if self.config.packet_loss_percent > 0.0 {
            let random = getrandom_u64() % 100;
            return (random as f64) < self.config.packet_loss_percent;
        }
        false
    }

    pub fn record_failure(&self) {
        self.failure_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn failure_count(&self) -> u64 {
        self.failure_count.load(Ordering::SeqCst)
    }

    pub fn inject_latency(&self) {
        if self.is_active() && self.config.latency_ms > 0 {
            std::thread::sleep(Duration::from_millis(self.config.latency_ms));
        }
    }
}

fn getrandom_u64() -> u64 {
    let mut buf = [0u8; 8];
    getrandom::getrandom(&mut buf).expect("CSPRNG failure");
    u64::from_le_bytes(buf)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_chaos_monkey_lifecycle() {
        let monkey = ChaosMonkey::new(ChaosConfig::default());
        assert!(!monkey.is_active());
        monkey.activate();
        assert!(monkey.is_active());
        monkey.deactivate();
        assert!(!monkey.is_active());
    }

    #[test]
    fn test_chaos_monkey_failure_counting() {
        let monkey = ChaosMonkey::new(ChaosConfig::default());
        monkey.activate();
        monkey.record_failure();
        monkey.record_failure();
        assert_eq!(monkey.failure_count(), 2);
    }

    #[test]
    fn test_packet_loss_injection() {
        let config = ChaosConfig {
            packet_loss_percent: 100.0,
            ..Default::default()
        };
        let monkey = ChaosMonkey::new(config);
        monkey.activate();
        assert!(monkey.should_inject_failure());
        monkey.deactivate();
        assert!(!monkey.should_inject_failure());
    }

    #[test]
    fn test_no_packet_loss_when_inactive() {
        let config = ChaosConfig {
            packet_loss_percent: 100.0,
            ..Default::default()
        };
        let monkey = ChaosMonkey::new(config);
        assert!(!monkey.should_inject_failure());
    }
}
