use kcm_core::types::KcmError;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct WalCheckpoint {
    pub offset: u64,
    pub timestamp: i64,
    pub entry_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WalState {
    Fresh,
    Active,
    Checkpointing,
    Replaying,
    Truncated,
    Error(String),
}

pub struct WalStateMachine {
    state: WalState,
    checkpoint_interval: Duration,
    last_checkpoint: Option<Instant>,
    entries_since_checkpoint: u64,
    max_entries_before_checkpoint: u64,
}

impl WalStateMachine {
    pub fn new() -> Self {
        WalStateMachine {
            state: WalState::Fresh,
            checkpoint_interval: Duration::from_secs(60),
            last_checkpoint: None,
            entries_since_checkpoint: 0,
            max_entries_before_checkpoint: 10_000,
        }
    }

    pub fn with_checkpoint_interval(mut self, interval: Duration) -> Self {
        self.checkpoint_interval = interval;
        self
    }

    pub fn state(&self) -> &WalState {
        &self.state
    }

    pub fn should_checkpoint(&self) -> bool {
        if let Some(last) = self.last_checkpoint {
            if last.elapsed() > self.checkpoint_interval {
                return true;
            }
        }
        self.entries_since_checkpoint >= self.max_entries_before_checkpoint
    }

    pub fn record_entry(&mut self) {
        self.entries_since_checkpoint += 1;
        self.state = WalState::Active;
    }

    pub fn begin_checkpoint(&mut self) {
        self.state = WalState::Checkpointing;
    }

    pub fn complete_checkpoint(&mut self) {
        self.last_checkpoint = Some(Instant::now());
        self.entries_since_checkpoint = 0;
        self.state = WalState::Active;
    }

    pub fn begin_replay(&mut self) {
        self.state = WalState::Replaying;
    }

    pub fn complete_replay(&mut self) {
        self.last_checkpoint = Some(Instant::now());
        self.entries_since_checkpoint = 0;
        self.state = WalState::Truncated;
    }

    pub fn error(&mut self, msg: String) {
        self.state = WalState::Error(msg);
    }

    pub fn transition(&mut self, target: WalState) -> Result<(), KcmError> {
        let valid = match (&self.state, &target) {
            (WalState::Fresh, WalState::Active) => true,
            (WalState::Active, WalState::Checkpointing) => true,
            (WalState::Checkpointing, WalState::Active) => true,
            (WalState::Fresh, WalState::Replaying) => true,
            (WalState::Replaying, WalState::Truncated) => true,
            (WalState::Active, WalState::Replaying) => true,
            (WalState::Replaying, WalState::Error(_)) => true,
            (WalState::Error(_), _) => false,
            (WalState::Truncated, WalState::Active) => true,
            _ => false,
        };

        if valid {
            self.state = target;
            Ok(())
        } else {
            Err(KcmError::InvalidArgument(format!(
                "Invalid WAL state transition: {:?} -> {:?}",
                self.state, target
            )))
        }
    }
}

impl Default for WalStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
