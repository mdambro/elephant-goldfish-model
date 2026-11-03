//! Composition root: builds the infrastructure, the use cases, and the adapters, then serves REST and gRPC.

mod grpc;
mod http;
mod persistence;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use application::ports::UserRepository;
use application::use_cases::RegisterUserUseCase;
use deadpool_postgres::{Config, Runtime};
use proto::users::v1::user_service_server::UserServiceServer;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configuration
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    let http_addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let grpc_addr: SocketAddr = "0.0.0.0:50051".parse()?;

    // Infrastructure: a connection pool (tokio-postgres has none of its own)
    let mut config = Config::new();
    config.url = Some(database_url);
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    let users: Arc<dyn UserRepository> = Arc::new(persistence::PgUserRepository::new(pool));

    // Use cases
    let register_user = Arc::new(RegisterUserUseCase::new(users));

    // Adapters: REST and gRPC on separate listeners
    let rest = http::router(http::AppState { register_user: Arc::clone(&register_user) });
    let grpc_service = UserServiceServer::new(grpc::GrpcUserService::new(register_user));

    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    let rest_server = async { axum::serve(listener, rest).await.context("REST server failed") };
    let grpc_server = async {
        tonic::transport::Server::builder()
            .add_service(grpc_service)
            .serve(grpc_addr)
            .await
            .context("gRPC server failed")
    };

    println!("REST on http://{http_addr}, gRPC on {grpc_addr}");
    tokio::try_join!(rest_server, grpc_server)?;
    Ok(())
}
