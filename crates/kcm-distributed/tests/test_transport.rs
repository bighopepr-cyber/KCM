#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_distributed::coordinator::{
    ParticipantTransport, TransactionCoordinator, TransactionStatus,
};
use kcm_distributed::transport::{InMemoryTransport, TcpTransport};
use std::sync::Arc;

#[test]
fn test_in_memory_transport_basic() {
    let transport = InMemoryTransport::new();
    transport.prepare(0, "txn-1");
    transport.commit(0, "txn-1");
    transport.abort(1, "txn-2");

    let votes = transport.get_votes();
    assert_eq!(votes.len(), 3);
    assert!(votes.iter().any(|(m, _)| m.contains("PREPARE")));
    assert!(votes.iter().any(|(m, _)| m.contains("COMMIT")));
    assert!(votes.iter().any(|(m, v)| m.contains("ABORT") && !v));
}

#[test]
fn test_in_memory_transport_as_2pc() {
    let transport = Arc::new(InMemoryTransport::new());
    let coord = TransactionCoordinator::with_transport(transport.clone());
    let txn_id = coord.begin_transaction(vec![0, 1, 2]);
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_ok());
    assert_eq!(
        coord.get_status(&txn_id),
        Some(TransactionStatus::Committed)
    );

    let votes = transport.get_votes();
    assert!(votes.len() >= 3);
    let prepare_count = votes.iter().filter(|(m, _)| m.contains("PREPARE")).count();
    assert_eq!(prepare_count, 3);
    let commit_count = votes.iter().filter(|(m, _)| m.contains("COMMIT")).count();
    assert_eq!(commit_count, 3);
}

#[test]
fn test_in_memory_transport_abort() {
    let transport = Arc::new(InMemoryTransport::new());
    let coord = TransactionCoordinator::with_transport(transport.clone());
    let txn_id = coord.begin_transaction(vec![0, 1]);
    coord.abort(&txn_id).unwrap();
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_tcp_transport_creation() {
    let transport = TcpTransport::new(vec![
        ("127.0.0.1".to_string(), 8000),
        ("127.0.0.1".to_string(), 8001),
    ])
    .with_timeout(std::time::Duration::from_secs(1))
    .with_retries(2);

    let prepared = transport.prepare(0, "txn-1");
    assert!(
        !prepared,
        "TCP transport should fail to connect to non-existent server"
    );
}

#[test]
fn test_tcp_transport_invalid_endpoint() {
    let transport = TcpTransport::new(vec![]);
    assert!(!transport.prepare(0, "txn-1"));
}
