use actix_web::{test, web, App, HttpResponse};
use kcm_core::types::*;
use kcm_interface::rest_api::*;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::health::HealthCheck;
use kcm_runtime::metrics::Metrics;
use serde::Deserialize;
use std::sync::Arc;

fn create_test_state() -> Arc<ApiState> {
    let db = KnowledgeDatabase::new().unwrap();
    let metrics = Arc::new(Metrics::new());
    let health_check = Arc::new(HealthCheck::new(metrics.clone()));
    let audit_log = Arc::new(kcm_security::audit::AuditLog::new());
    Arc::new(ApiState {
        db: Arc::new(db),
        metrics,
        health_check,
        audit_log: Some(audit_log),
    })
}

fn build_response(response: ApiResponse) -> HttpResponse {
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn health_handler(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    build_response(handle_health(&state))
}

async fn insert_handler(
    state: web::Data<Arc<ApiState>>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let subject = body["subject"].as_u64().unwrap_or(0) as u32;
    let predicate = body["predicate"].as_u64().unwrap_or(0) as u8;
    let object = body["object"].as_u64().unwrap_or(0) as u32;
    let confidence = body["confidence"].as_f64().unwrap_or(0.0);
    build_response(handle_insert(
        &state, subject, predicate, object, confidence,
    ))
}

#[derive(Deserialize)]
struct QueryParams {
    subject: Option<u32>,
    predicate: Option<u8>,
    object: Option<u32>,
    confidence_min: Option<f64>,
}

async fn query_handler(
    state: web::Data<Arc<ApiState>>,
    params: web::Query<QueryParams>,
) -> HttpResponse {
    build_response(handle_query(
        &state,
        params.subject,
        params.predicate,
        params.object,
        params.confidence_min,
    ))
}

async fn get_fact_handler(state: web::Data<Arc<ApiState>>, path: web::Path<u64>) -> HttpResponse {
    build_response(handle_get_fact(&state, *path))
}

async fn update_handler(
    state: web::Data<Arc<ApiState>>,
    path: web::Path<u64>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let subject = body["subject"].as_u64().unwrap_or(0) as u32;
    let predicate = body["predicate"].as_u64().unwrap_or(0) as u8;
    let object = body["object"].as_u64().unwrap_or(0) as u32;
    let confidence = body["confidence"].as_f64().unwrap_or(0.0);
    build_response(handle_update(
        &state, *path, subject, predicate, object, confidence,
    ))
}

async fn delete_handler(state: web::Data<Arc<ApiState>>, path: web::Path<u64>) -> HttpResponse {
    build_response(handle_delete(&state, *path))
}

async fn stats_handler(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    build_response(handle_stats(&state))
}

#[actix_web::test]
async fn test_health_endpoint_200() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/health", web::get().to(health_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_web::test]
async fn test_insert_endpoint_201() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!({
            "subject": 1, "predicate": 0, "object": 2, "confidence": 0.9
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "created");
}

#[actix_web::test]
async fn test_insert_invalid_confidence() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!({
            "subject": 1, "predicate": 0, "object": 2, "confidence": 1.5
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_insert_negative_confidence() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!({
            "subject": 1, "predicate": 0, "object": 2, "confidence": -0.1
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_query_endpoint_200() {
    let state = create_test_state();
    state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::get().to(query_handler)),
    )
    .await;
    let req = test::TestRequest::get()
        .uri("/facts?subject=1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 1);
}

#[actix_web::test]
async fn test_query_empty_database() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::get().to(query_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/facts").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 0);
}

#[actix_web::test]
async fn test_query_with_predicate_filter() {
    let state = create_test_state();
    state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(1), ObjectID(3), 0.8).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::get().to(query_handler)),
    )
    .await;
    let req = test::TestRequest::get()
        .uri("/facts?subject=1&predicate=0")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 1);
}

#[actix_web::test]
async fn test_get_fact_not_found() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts/{id}", web::get().to(get_fact_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/facts/999").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_fact_found() {
    let state = create_test_state();
    let row_id = state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts/{id}", web::get().to(get_fact_handler)),
    )
    .await;
    let uri = format!("/facts/{}", row_id.0);
    let req = test::TestRequest::get().uri(&uri).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["subject"], 1);
}

#[actix_web::test]
async fn test_update_endpoint() {
    let state = create_test_state();
    let row_id = state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts/{id}", web::put().to(update_handler)),
    )
    .await;
    let uri = format!("/facts/{}", row_id.0);
    let req = test::TestRequest::put()
        .uri(&uri)
        .set_json(serde_json::json!({
            "subject": 5, "predicate": 2, "object": 6, "confidence": 0.7
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "updated");
}

#[actix_web::test]
async fn test_delete_endpoint() {
    let state = create_test_state();
    let row_id = state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/facts/{id}", web::delete().to(delete_handler)),
    )
    .await;
    let uri = format!("/facts/{}", row_id.0);
    let req = test::TestRequest::delete().uri(&uri).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(state.db.active_fact_count(), 0);
}

#[actix_web::test]
async fn test_stats_endpoint() {
    let state = create_test_state();
    state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/stats", web::get().to(stats_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/stats").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("total_queries").is_some());
}

#[actix_web::test]
async fn test_insert_query_delete_cycle() {
    let state = create_test_state();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    state.db.insert(&fact).unwrap();
    assert_eq!(state.db.fact_count(), 1);

    let results = state
        .db
        .query()
        .with_subject(SubjectID(1))
        .execute()
        .unwrap();
    assert_eq!(results.len(), 1);

    state.db.delete(RowID(0)).unwrap();
    assert_eq!(state.db.active_fact_count(), 0);
}

#[actix_web::test]
async fn test_multiple_inserts_and_query() {
    let state = create_test_state();
    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9).unwrap();
        state.db.insert(&fact).unwrap();
    }
    assert_eq!(state.db.fact_count(), 10);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::get().to(query_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/facts").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 10);
}

#[actix_web::test]
async fn test_update_nonexistent_fact() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts/{id}", web::put().to(update_handler)),
    )
    .await;
    let req = test::TestRequest::put()
        .uri("/facts/999")
        .set_json(serde_json::json!({
            "subject": 5, "predicate": 2, "object": 6, "confidence": 0.7
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 500);
}

#[actix_web::test]
async fn test_delete_nonexistent_fact() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts/{id}", web::delete().to(delete_handler)),
    )
    .await;
    let req = test::TestRequest::delete().uri("/facts/999").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 500);
}

#[actix_web::test]
async fn test_health_contains_json_fields() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/health", web::get().to(health_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_web::test]
async fn test_insert_boundary_confidence_zero() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!({
            "subject": 1, "predicate": 0, "object": 2, "confidence": 0.0
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[actix_web::test]
async fn test_insert_boundary_confidence_one() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!({
            "subject": 1, "predicate": 0, "object": 2, "confidence": 1.0
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

// ============= BATCH INSERT TESTS =============

async fn batch_insert_handler(
    state: web::Data<Arc<ApiState>>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let mut tuples = Vec::new();
    if let Some(facts) = body.as_array() {
        for f in facts {
            let s = f["subject"].as_u64().unwrap_or(0) as u32;
            let p = f["predicate"].as_u64().unwrap_or(0) as u8;
            let o = f["object"].as_u64().unwrap_or(0) as u32;
            let c = f["confidence"].as_f64().unwrap_or(0.5);
            tuples.push((s, p, o, c));
        }
    }
    build_response(handle_batch_insert(&state, tuples))
}

async fn openapi_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(kcm_interface::openapi::openapi_spec())
}

async fn metrics_handler(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    let snap = state.metrics.snapshot();
    let body = format!(
        "kcm_queries_total {}\nkcm_inserts_total {}\nkcm_memory_bytes {}",
        snap.queries_total, snap.inserts_total, snap.memory_bytes
    );
    HttpResponse::Ok().content_type("text/plain").body(body)
}

#[actix_web::test]
async fn test_batch_insert() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(batch_insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!([
            {"subject": 1, "predicate": 0, "object": 2, "confidence": 0.95},
            {"subject": 2, "predicate": 1, "object": 3, "confidence": 0.90},
            {"subject": 3, "predicate": 2, "object": 4, "confidence": 0.85}
        ]))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["inserted"], 3);
    assert_eq!(body["errors"], 0);
}

#[actix_web::test]
async fn test_batch_insert_with_errors() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(batch_insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!([
            {"subject": 1, "predicate": 0, "object": 2, "confidence": 0.95},
            {"subject": 2, "predicate": 0, "object": 3, "confidence": 2.0}
        ]))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["inserted"], 1);
    assert_eq!(body["errors"], 1);
}

#[actix_web::test]
async fn test_batch_insert_empty() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(batch_insert_handler)),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!([]))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["inserted"], 0);
}

#[actix_web::test]
async fn test_openapi_endpoint() {
    let app =
        test::init_service(App::new().route("/openapi.json", web::get().to(openapi_handler))).await;
    let req = test::TestRequest::get().uri("/openapi.json").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["openapi"], "3.1.0");
    assert_eq!(body["info"]["title"], "KCM Knowledge Columnar Model API");
}

#[actix_web::test]
async fn test_metrics_endpoint() {
    let state = create_test_state();
    state
        .db
        .insert(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/metrics", web::get().to(metrics_handler)),
    )
    .await;
    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body = {
        let bytes = test::read_body(resp).await;
        String::from_utf8(bytes.to_vec()).unwrap_or_default()
    };
    assert!(body.contains("kcm_queries_total"));
    assert!(body.contains("kcm_inserts_total"));
    assert!(body.contains("kcm_memory_bytes"));
}

#[actix_web::test]
async fn test_insert_then_batch_query() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(batch_insert_handler))
            .route("/facts", web::get().to(query_handler)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(serde_json::json!([
            {"subject": 1, "predicate": 0, "object": 100, "confidence": 0.95},
            {"subject": 1, "predicate": 1, "object": 200, "confidence": 0.90},
            {"subject": 2, "predicate": 0, "object": 300, "confidence": 0.85}
        ]))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let req = test::TestRequest::get().uri("/facts").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 3);

    let req = test::TestRequest::get()
        .uri("/facts?subject=1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 2);

    let req = test::TestRequest::get()
        .uri("/facts?confidence_min=0.9")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 2);
}
