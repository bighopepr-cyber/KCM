use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use crate::coordinator::ParticipantTransport;

/// TCP-based transport for distributed 2PC communication.
/// Supports configurable timeout and retry policies.
pub struct TcpTransport {
    endpoints: Vec<(String, u16)>,
    timeout: Duration,
    max_retries: usize,
}

impl TcpTransport {
    pub fn new(endpoints: Vec<(String, u16)>) -> Self {
        TcpTransport {
            endpoints,
            timeout: Duration::from_secs(5),
            max_retries: 3,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    fn send_message(&self, participant_id: usize, message: &[u8]) -> bool {
        if participant_id >= self.endpoints.len() {
            return false;
        }
        let (host, port) = &self.endpoints[participant_id];
        let addr: SocketAddr = match format!("{}:{}", host, port).parse() {
            Ok(a) => a,
            Err(_) => return false,
        };

        for attempt in 0..self.max_retries {
            match TcpStream::connect_timeout(&addr, self.timeout) {
                Ok(mut stream) => {
                    stream.set_nodelay(true).ok();
                    if stream.write_all(message).is_ok() {
                        let mut response = [0u8; 1];
                        stream.set_read_timeout(Some(self.timeout)).ok();
                        if stream.read_exact(&mut response).is_ok() {
                            return response[0] == 1;
                        }
                    }
                }
                Err(_) => {
                    if attempt == self.max_retries - 1 {
                        return false;
                    }
                    std::thread::sleep(Duration::from_millis(100 * (attempt + 1) as u64));
                }
            }
        }
        false
    }
}

impl ParticipantTransport for TcpTransport {
    fn prepare(&self, participant_id: usize, txn_id: &str) -> bool {
        let msg = format!("PREPARE {}", txn_id);
        self.send_message(participant_id, msg.as_bytes())
    }

    fn commit(&self, participant_id: usize, txn_id: &str) {
        let msg = format!("COMMIT {}", txn_id);
        let _ = self.send_message(participant_id, msg.as_bytes());
    }

    fn abort(&self, participant_id: usize, txn_id: &str) {
        let msg = format!("ABORT {}", txn_id);
        let _ = self.send_message(participant_id, msg.as_bytes());
    }
}

type VoteMap = HashMap<usize, Vec<(String, bool)>>;

/// In-memory transport for testing — simulates network without real I/O.
pub struct InMemoryTransport {
    votes: Arc<parking_lot::Mutex<VoteMap>>,
}

impl InMemoryTransport {
    pub fn new() -> Self {
        InMemoryTransport {
            votes: Arc::new(parking_lot::Mutex::new(HashMap::new())),
        }
    }

    pub fn get_votes(&self) -> Vec<(String, bool)> {
        let votes = self.votes.lock();
        let mut all = Vec::new();
        for msgs in votes.values() {
            for msg in msgs {
                all.push(msg.clone());
            }
        }
        all
    }
}

impl Default for InMemoryTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticipantTransport for InMemoryTransport {
    fn prepare(&self, participant_id: usize, txn_id: &str) -> bool {
        self.votes
            .lock()
            .entry(participant_id)
            .or_default()
            .push((format!("PREPARE {}", txn_id), true));
        true
    }

    fn commit(&self, participant_id: usize, txn_id: &str) {
        self.votes
            .lock()
            .entry(participant_id)
            .or_default()
            .push((format!("COMMIT {}", txn_id), true));
    }

    fn abort(&self, participant_id: usize, txn_id: &str) {
        self.votes
            .lock()
            .entry(participant_id)
            .or_default()
            .push((format!("ABORT {}", txn_id), false));
    }
}
