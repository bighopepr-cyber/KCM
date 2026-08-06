use kcm_runtime::database::KnowledgeDatabase;
use std::sync::Arc;

mod grpc_server;

use grpc_server::knowledge_service::knowledge_service_server::KnowledgeServiceServer;
use grpc_server::KcmGrpcService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let db = KnowledgeDatabase::new().expect("Failed to create database");
    let grpc_addr = std::env::var("KCM_GRPC_ADDR").unwrap_or_else(|_| "0.0.0.0:50051".to_string());

    let grpc_service = KcmGrpcService { db: Arc::new(db) };

    log::info!("Starting gRPC server on {}", grpc_addr);

    tonic::transport::Server::builder()
        .add_service(KnowledgeServiceServer::new(grpc_service))
        .serve(grpc_addr.parse()?)
        .await?;

    Ok(())
}
