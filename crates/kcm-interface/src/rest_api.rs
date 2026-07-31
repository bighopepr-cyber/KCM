use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::health::HealthCheck;
use kcm_runtime::metrics::Metrics;
use std::sync::Arc;

pub struct ApiState {
    pub db: Arc<KnowledgeDatabase>,
    pub metrics: Arc<Metrics>,
    pub health_check: Arc<HealthCheck>,
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
            body: format!(r#"{{"error":"{}"}}"#, msg),
        }
    }
    pub fn not_found(msg: &str) -> Self {
        ApiResponse {
            status: 404,
            body: format!(r#"{{"error":"{}"}}"#, msg),
        }
    }
    pub fn internal_error(msg: &str) -> Self {
        ApiResponse {
            status: 500,
            body: format!(r#"{{"error":"{}"}}"#, msg),
        }
    }
}

pub fn handle_health(state: &ApiState) -> ApiResponse {
    let report = state.health_check.check_detailed();
    ApiResponse::ok(&report.to_json())
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
        Err(e) => return ApiResponse::bad_request(&e),
    };

    match state.db.insert(&fact) {
        Ok(row_id) => {
            state.metrics.record_insert(true);
            ApiResponse::created(&format!(r#"{{"row_id":{},"status":"OK"}}"#, row_id.0))
        }
        Err(e) => {
            state.metrics.record_insert(false);
            ApiResponse::internal_error(&e.to_string())
        }
    }
}

pub fn handle_query(
    state: &ApiState,
    subject: Option<u32>,
    predicate: Option<u8>,
    object: Option<u32>,
    confidence_min: Option<f64>,
) -> ApiResponse {
    let start = std::time::Instant::now();
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

    match query.execute() {
        Ok(facts) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            state.metrics.record_query(duration_ms, true);

            let fact_data: Vec<String> = facts.iter().map(|f| {
                format!(
                    r#"{{"subject":{},"predicate":{},"object":{},"confidence":{},"timestamp":{}}}"#,
                    f.subject.0, f.predicate.0, f.object.0, f.confidence, f.timestamp
                )
            }).collect();

            ApiResponse::ok(&format!(
                r#"{{"facts":[{}],"total_count":{}}}"#,
                fact_data.join(","),
                facts.len()
            ))
        }
        Err(e) => {
            state.metrics.record_query(0, false);
            ApiResponse::internal_error(&e.to_string())
        }
    }
}

pub fn handle_get_fact(state: &ApiState, row_id: u64) -> ApiResponse {
    match state.db.get_fact(RowID(row_id)) {
        Ok(Some(fact)) => ApiResponse::ok(&format!(
            r#"{{"subject":{},"predicate":{},"object":{},"confidence":{},"timestamp":{}}}"#,
            fact.subject.0, fact.predicate.0, fact.object.0, fact.confidence, fact.timestamp
        )),
        Ok(None) => ApiResponse::not_found("Fact not found"),
        Err(e) => ApiResponse::internal_error(&e.to_string()),
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
        Err(e) => return ApiResponse::bad_request(&e),
    };

    match state.db.update(RowID(row_id), &fact) {
        Ok(()) => ApiResponse::ok(r#"{"status":"updated"}"#),
        Err(e) => ApiResponse::internal_error(&e.to_string()),
    }
}

pub fn handle_delete(state: &ApiState, row_id: u64) -> ApiResponse {
    match state.db.delete(RowID(row_id)) {
        Ok(()) => ApiResponse::ok(r#"{"status":"deleted"}"#),
        Err(e) => ApiResponse::internal_error(&e.to_string()),
    }
}

pub fn handle_stats(state: &ApiState) -> ApiResponse {
    let snap = state.metrics.snapshot();
    ApiResponse::ok(&snap.to_json())
}
