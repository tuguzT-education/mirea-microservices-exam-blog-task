#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{Extension, Router, Server};
use dotenv::dotenv;
use exam_task::{
    di::app_module,
    route::{health, task},
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv().with_context(|| ".env file not found")?;
    }
    let log_directives = std::env::var("RUST_LOG").with_context(|| "RUST_LOG must be set")?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_directives))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .with_context(|| "failed to init tracing")?;

    let database_url = std::env::var("DATABASE_URL").with_context(|| "DATABASE_URL must be set")?;
    let module = app_module(database_url)?;
    let module = Arc::new(module);

    let app = Router::new()
        .merge(task::all())
        .layer(Extension(module))
        .merge(health::health())
        .layer(TraceLayer::new_for_http());
    let addr = &SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}
