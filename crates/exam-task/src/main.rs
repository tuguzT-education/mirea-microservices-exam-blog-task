#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{Extension, Router, Server};
use dotenv::dotenv;
use exam_task::{
    di::{app_module, FilterTaskUseCase},
    route::{health, task},
    schedule::Scheduler,
};
use shaku::HasComponent;
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

    let database_uri = std::env::var("DATABASE_URI").with_context(|| "DATABASE_URI must be set")?;
    let post_service_url =
        std::env::var("POST_SERVICE_URL").with_context(|| "POST_SERVICE_URL must be set")?;
    let module = app_module(database_uri, post_service_url).build();
    let module = Arc::new(module);

    setup_publish_tasks(module.resolve_ref(), module.resolve_ref()).await?;

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

async fn setup_publish_tasks(scheduler: &Scheduler, tasks: &FilterTaskUseCase) -> Result<()> {
    let tasks = tasks.filter_task(Default::default()).await?;
    for task in tasks {
        scheduler.set_publish_task_state(task).await;
    }
    Ok(())
}
