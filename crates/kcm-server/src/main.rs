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

async fn metrics_handler(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    let snap = state.metrics.snapshot();
    let body = format!(
        "# HELP kcm_queries_total Total number of queries\n\
         # TYPE kcm_queries_total counter\n\
         kcm_queries_total {}\n\
         # HELP kcm_queries_failed_total Total number of failed queries\n\
         # TYPE kcm_queries_failed_total counter\n\
         kcm_queries_failed_total {}\n\
         # HELP kcm_query_avg_latency_ms Average query latency in milliseconds\n\
         # TYPE kcm_query_avg_latency_ms gauge\n\
         kcm_query_avg_latency_ms {:.2}\n\
         # HELP kcm_inserts_total Total number of inserts\n\
         # TYPE kcm_inserts_total counter\n\
         kcm_inserts_total {}\n\
         # HELP kcm_inserts_failed_total Total number of failed inserts\n\
         # TYPE kcm_inserts_failed_total counter\n\
         kcm_inserts_failed_total {}\n\
         # HELP kcm_cache_hit_ratio Cache hit ratio\n\
         # TYPE kcm_cache_hit_ratio gauge\n\
         kcm_cache_hit_ratio {:.4}\n\
         # HELP kcm_memory_bytes Memory usage estimate\n\
         # TYPE kcm_memory_bytes gauge\n\
         kcm_memory_bytes {}\n",
        snap.queries_total,
        snap.queries_failed,
        snap.avg_query_latency_ms,
        snap.inserts_total,
        snap.inserts_failed,
        snap.cache_hit_ratio,
        snap.memory_bytes,
    );
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(body)
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
    log::info!("Starting KCM HTTP server on {}", bind_addr);

    let state_clone = state.clone();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state_clone.clone()))
            .route("/health", web::get().to(health_handler))
            .route("/facts", web::post().to(insert_handler))
            .route("/facts", web::get().to(query_handler))
            .route("/facts/{id}", web::get().to(get_fact_handler))
            .route("/facts/{id}", web::put().to(update_handler))
            .route("/facts/{id}", web::delete().to(delete_handler))
            .route("/stats", web::get().to(stats_handler))
            .route("/metrics", web::get().to(metrics_handler))
    })
    .bind(&bind_addr)?;

    let server_handle = server.run();

    // Graceful shutdown: run until signal received, then stop accepting new connections
    let server_handle_clone = server_handle.handle();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                log::info!("Received shutdown signal, initiating graceful shutdown...");
                server_handle_clone.stop(true).await;
                log::info!("Server gracefully stopped.");
            }
            Err(e) => {
                log::error!("Signal handler error: {}", e);
            }
        }
    });

    server_handle.await
}
