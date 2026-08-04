use actix_web::{test, web, App};
use kcm_interface::rest_api::ApiState;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::health::HealthCheck;
use kcm_runtime::metrics::Metrics;
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

async fn health_handler(state: web::Data<Arc<ApiState>>) -> actix_web::HttpResponse {
    let response = kcm_interface::rest_api::handle_health(&state);
    actix_web::HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn insert_handler(
    state: web::Data<Arc<ApiState>>,
    body: web::Json<serde_json::Value>,
) -> actix_web::HttpResponse {
    let subject = body["subject"].as_u64().unwrap_or(0) as u32;
    let predicate = body["predicate"].as_u64().unwrap_or(0) as u8;
    let object = body["object"].as_u64().unwrap_or(0) as u32;
    let confidence = body["confidence"].as_f64().unwrap_or(0.0);
    let response =
        kcm_interface::rest_api::handle_insert(&state, subject, predicate, object, confidence);
    actix_web::HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn stats_handler(state: web::Data<Arc<ApiState>>) -> actix_web::HttpResponse {
    let response = kcm_interface::rest_api::handle_stats(&state);
    actix_web::HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

#[actix_web::test]
async fn test_health_endpoint() {
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
}

#[actix_web::test]
async fn test_insert_and_query() {
    let state = create_test_state();

    let fact = kcm_core::types::Fact::new(
        kcm_core::types::SubjectID(1),
        kcm_core::types::PredicateID(0),
        kcm_core::types::ObjectID(2),
        0.9,
    )
    .unwrap();
    state.db.insert(&fact).unwrap();

    let results = state.db.query().execute().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].subject, kcm_core::types::SubjectID(1));
}

#[actix_web::test]
async fn test_insert_endpoint() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/facts", web::post().to(insert_handler)),
    )
    .await;

    let body = serde_json::json!({
        "subject": 1,
        "predicate": 0,
        "object": 2,
        "confidence": 0.9
    });

    let req = test::TestRequest::post()
        .uri("/facts")
        .set_json(&body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[actix_web::test]
async fn test_stats_endpoint() {
    let state = create_test_state();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .route("/stats", web::get().to(stats_handler)),
    )
    .await;

    let req = test::TestRequest::get().uri("/stats").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}
