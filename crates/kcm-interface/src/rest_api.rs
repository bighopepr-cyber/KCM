use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::health::HealthCheck;
use kcm_runtime::metrics::Metrics;
use kcm_security::audit::AuditLog;
use std::sync::Arc;

pub struct ApiState {
    pub db: Arc<KnowledgeDatabase>,
    pub metrics: Arc<Metrics>,
    pub health_check: Arc<HealthCheck>,
    pub audit_log: Option<Arc<AuditLog>>,
}

pub struct ApiResponse {
    pub status: u16,
    pub body: String,
}

impl ApiResponse {
    pub fn ok(body: &str) -> Self {
        ApiResponse {
            status: 200,
            body: body.to_string(),
        }
    }

    pub fn created(body: &str) -> Self {
        ApiResponse {
            status: 201,
            body: body.to_string(),
        }
    }

    pub fn bad_request(msg: &str) -> Self {
        ApiResponse {
            status: 400,
            body: format!(r#"{{"error":"{}","status":400}}"#, msg),
        }
    }

    pub fn not_found(msg: &str) -> Self {
        ApiResponse {
            status: 404,
            body: format!(r#"{{"error":"{}","status":404}}"#, msg),
        }
    }

    pub fn internal_error(msg: &str) -> Self {
        ApiResponse {
            status: 500,
            body: format!(r#"{{"error":"{}","status":500}}"#, msg),
        }
    }

    pub fn rate_limited() -> Self {
        ApiResponse {
            status: 429,
            body: r#"{"error":"Rate limit exceeded","status":429}"#.to_string(),
        }
    }

    pub fn from_kcm_error(err: &KcmError) -> Self {
        match err {
            KcmError::NotFound(msg) => ApiResponse::not_found(msg),
            KcmError::OutOfMemory => ApiResponse {
                status: 507,
                body: r#"{"error":"Out of memory","status":507}"#.to_string(),
            },
            KcmError::Conflict(msg) => ApiResponse {
                status: 409,
                body: format!(r#"{{"error":"{}","status":409}}"#, msg),
            },
            KcmError::TransactionAborted => ApiResponse {
                status: 409,
                body: r#"{"error":"Transaction aborted","status":409}"#.to_string(),
            },
            KcmError::InvalidArgument(msg) => ApiResponse::bad_request(msg),
            KcmError::Corrupted(_) => ApiResponse::internal_error("Data corruption detected"),
            KcmError::Io(_) => ApiResponse::internal_error("An I/O error occurred"),
        }
    }
}

pub fn handle_health(state: &ApiState) -> ApiResponse {
    use kcm_runtime::health::HealthStatus;
    match state.health_check.check() {
        HealthStatus::Healthy => ApiResponse::ok(r#"{"status":"healthy"}"#),
        HealthStatus::Degraded => ApiResponse::ok(r#"{"status":"degraded"}"#),
        HealthStatus::Unhealthy => ApiResponse::internal_error("Health check failed"),
    }
}

pub fn handle_insert(
    state: &ApiState,
    subject: u32,
    predicate: u8,
    object: u32,
    confidence: f64,
) -> ApiResponse {
    let fact = match Fact::new(
        SubjectID(subject),
        PredicateID(predicate),
        ObjectID(object),
        confidence,
    ) {
        Ok(f) => f,
        Err(e) => return ApiResponse::bad_request(&format!("Invalid fact: {}", e)),
    };
    match state.db.insert(&fact) {
        Ok(row_id) => {
            state.metrics.record_insert(true);
            if let Some(ref log) = state.audit_log {
                let _ = log.log_insert("api", row_id.0);
            }
            ApiResponse::created(&format!(r#"{{"row_id":{},"status":"created"}}"#, row_id.0))
        }
        Err(e) => {
            state.metrics.record_insert(false);
            ApiResponse::from_kcm_error(&e)
        }
    }
}

pub fn handle_batch_insert(state: &ApiState, facts: Vec<(u32, u8, u32, f64)>) -> ApiResponse {
    let mut inserted = 0u64;
    let mut errors = 0u64;
    for (s, p, o, c) in facts {
        if let Ok(fact) = Fact::new(SubjectID(s), PredicateID(p), ObjectID(o), c) {
            match state.db.insert(&fact) {
                Ok(_) => inserted += 1,
                Err(_) => errors += 1,
            }
        } else {
            errors += 1;
        }
    }
    state.metrics.record_insert(inserted > 0);
    ApiResponse::ok(&format!(
        r#"{{"inserted":{},"errors":{},"status":"ok"}}"#,
        inserted, errors
    ))
}

pub fn handle_query(
    state: &ApiState,
    subject: Option<u32>,
    predicate: Option<u8>,
    object: Option<u32>,
    confidence_min: Option<f64>,
    limit: Option<usize>,
) -> ApiResponse {
    let mut query = state.db.query();
    if let Some(s) = subject {
        query = query.with_subject(SubjectID(s));
    }
    if let Some(p) = predicate {
        query = query.with_predicate(PredicateID(p));
    }
    if let Some(o) = object {
        query = query.with_object(ObjectID(o));
    }
    if let Some(c) = confidence_min {
        query = query.with_confidence(c);
    }
    if let Some(limit) = limit {
        query = query.with_limit(limit);
    }

    match query.execute() {
        Ok(results) => {
            state.metrics.record_query(0, true);
            if let Some(ref log) = state.audit_log {
                let _ = log.log_query("api", "query");
            }
            let facts: Vec<String> = results
                .iter()
                .map(|f| {
                    format!(
                        r#"{{"subject":{},"predicate":{},"object":{},"confidence":{}}}"#,
                        f.subject.0, f.predicate.0, f.object.0, f.confidence
                    )
                })
                .collect();
            ApiResponse::ok(&format!(
                r#"{{"facts":[{}],"count":{}}}"#,
                facts.join(","),
                results.len()
            ))
        }
        Err(e) => {
            state.metrics.record_query(0, false);
            ApiResponse::from_kcm_error(&e)
        }
    }
}

pub fn handle_get_fact(state: &ApiState, row_id: u64) -> ApiResponse {
    match state.db.get_fact(RowID(row_id)) {
        Ok(Some(fact)) => ApiResponse::ok(&format!(
            r#"{{"row_id":{},"subject":{},"predicate":{},"object":{},"confidence":{}}}"#,
            row_id, fact.subject.0, fact.predicate.0, fact.object.0, fact.confidence
        )),
        Ok(None) => ApiResponse::not_found(&format!("Fact {} not found", row_id)),
        Err(e) => ApiResponse::from_kcm_error(&e),
    }
}

pub fn handle_update(
    state: &ApiState,
    row_id: u64,
    subject: u32,
    predicate: u8,
    object: u32,
    confidence: f64,
) -> ApiResponse {
    let fact = match Fact::new(
        SubjectID(subject),
        PredicateID(predicate),
        ObjectID(object),
        confidence,
    ) {
        Ok(f) => f,
        Err(e) => return ApiResponse::bad_request(&format!("Invalid fact: {}", e)),
    };
    match state.db.update(RowID(row_id), &fact) {
        Ok(()) => ApiResponse::ok(&format!(r#"{{"row_id":{},"status":"updated"}}"#, row_id)),
        Err(e) => ApiResponse::from_kcm_error(&e),
    }
}

pub fn handle_delete(state: &ApiState, row_id: u64) -> ApiResponse {
    match state.db.delete(RowID(row_id)) {
        Ok(()) => {
            if let Some(ref log) = state.audit_log {
                let _ = log.log_delete("api", row_id);
            }
            ApiResponse::ok(&format!(r#"{{"row_id":{},"status":"deleted"}}"#, row_id))
        }
        Err(e) => ApiResponse::from_kcm_error(&e),
    }
}

pub fn handle_stats(state: &ApiState) -> ApiResponse {
    let snapshot = state.metrics.snapshot();
    ApiResponse::ok(&format!(
        r#"{{"fact_count":{},"active_count":{},"total_inserts":{},"total_queries":{},"avg_latency_ms":{:.2},"estimated_memory_bytes":{}}}"#,
        state.db.fact_count(),
        state.db.active_fact_count(),
        snapshot.inserts_total,
        snapshot.queries_total,
        snapshot.avg_query_latency_ms,
        snapshot.estimated_memory_bytes
    ))
}
