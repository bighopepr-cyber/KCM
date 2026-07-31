use actix_web::{web, App, HttpResponse, HttpServer};
use kcm_interface::rest_api::{
    handle_delete, handle_get_fact, handle_health, handle_insert, handle_query, handle_stats,
    handle_update, ApiState,
};
use kcm_runtime::database::KnowledgeDatabase;
use kcm_runtime::health::HealthCheck;
use kcm_runtime::metrics::Metrics;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct InsertRequest {
    subject: u32,
    predicate: u8,
    object: u32,
    confidence: f64,
}

#[derive(Deserialize)]
struct UpdateRequest {
    subject: u32,
    predicate: u8,
    object: u32,
    confidence: f64,
}

#[derive(Deserialize)]
struct QueryParams {
    subject: Option<u32>,
    predicate: Option<u8>,
    object: Option<u32>,
    confidence_min: Option<f64>,
}

async fn health_handler(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    let response = handle_health(&state);
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn insert_handler(
    state: web::Data<Arc<ApiState>>,
    body: web::Json<InsertRequest>,
) -> HttpResponse {
    let response = handle_insert(
        &state,
        body.subject,
        body.predicate,
        body.object,
        body.confidence,
    );
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn query_handler(
    state: web::Data<Arc<ApiState>>,
    params: web::Query<QueryParams>,
) -> HttpResponse {
    let response = handle_query(
        &state,
        params.subject,
        params.predicate,
        params.object,
        params.confidence_min,
    );
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn get_fact_handler(state: web::Data<Arc<ApiState>>, path: web::Path<u64>) -> HttpResponse {
    let response = handle_get_fact(&state, *path);
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn update_handler(
    state: web::Data<Arc<ApiState>>,
    path: web::Path<u64>,
    body: web::Json<UpdateRequest>,
) -> HttpResponse {
    let response = handle_update(
        &state,
        *path,
        body.subject,
        body.predicate,
        body.object,
        body.confidence,
    );
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn delete_handler(state: web::Data<Arc<ApiState>>, path: web::Path<u64>) -> HttpResponse {
    let response = handle_delete(&state, *path);
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

async fn stats_handler(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    let response = handle_stats(&state);
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(response.status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .content_type("application/json")
    .body(response.body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let db = KnowledgeDatabase::new().expect("Failed to create database");
    let metrics = Arc::new(Metrics::new());
    let health_check = Arc::new(HealthCheck::new(metrics.clone()));

    let state = Arc::new(ApiState {
        db: Arc::new(db),
        metrics: metrics.clone(),
        health_check,
    });

    let bind_addr = std::env::var("KCM_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    log::info!("Starting KCM server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/health", web::get().to(health_handler))
            .route("/facts", web::post().to(insert_handler))
            .route("/facts", web::get().to(query_handler))
            .route("/facts/{id}", web::get().to(get_fact_handler))
            .route("/facts/{id}", web::put().to(update_handler))
            .route("/facts/{id}", web::delete().to(delete_handler))
            .route("/stats", web::get().to(stats_handler))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
