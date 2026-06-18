mod api;
mod course_repo;

use crate::course_repo::CourseRepo;
use axum::Extension;
use axum::Router;
use axum::http::{Method, header};
use axum::routing::get;
use clap::Parser;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use murmelbahn_lib::physical::SetRepo;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, info};

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(env)]
    pub sets_directory: PathBuf,
}

pub struct AppState {
    course_repo: CourseRepo,
    sets_repo: SetRepo,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::parse();

    // TODO: Print version
    info!("Murmelbahn Web starting up");

    // Database setup
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    info!("Connected with DB");

    let course_repo = CourseRepo::new(db);
    let mut sets_repo = SetRepo::new();
    sets_repo.read_directory(config.sets_directory)?;

    let shared_state = Arc::new(AppState {
        course_repo,
        sets_repo,
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let api_routes = api::router(shared_state.clone());

    let router = Router::new()
        .nest("/api", api_routes)
        .route("/metrics", get(metrics).layer(build_prometheus_extension()))
        .layer(cors.clone());

    let bind = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    debug!("listening on {}", bind);
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

/// This renders the metrics collected by the `metrics` crate into Prometheus compatible output.
pub async fn metrics(Extension(context): Extension<PrometheusHandle>) -> String {
    context.render()
}

/// Builds the [`Extension`] with the [`PrometheusHandle`] so we can access the metrics collected
/// by the `metrics` crate and output it in the Prometheus format.
fn build_prometheus_extension() -> Extension<PrometheusHandle> {
    let prometheus_builder = PrometheusBuilder::new();
    let prometheus_handle = prometheus_builder
        .install_recorder()
        .expect("failed to install recorder");
    Extension(prometheus_handle)
}
