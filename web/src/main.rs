mod buildable;
mod course;
mod course_repo;
mod set;

use crate::buildable::buildable;
use crate::course::{course_bom, course_dump};
use crate::course_repo::CourseRepo;
use crate::set::set_list;
use axum::http::{header, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, get_service, post};
use axum::{Extension, Router};
use clap::Parser;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use murmelbahn_lib::physical::SetRepo;
use sqlx::postgres::PgPoolOptions;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
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

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
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

    let serve_assets = ServeDir::new("frontend/dist/assets");
    let serve_assets = get_service(serve_assets).handle_error(handle_error);

    let serve_index = ServeFile::new("frontend/dist/index.html");
    let serve_index = get_service(serve_index).handle_error(handle_error);

    let course_routes = Router::new()
        .route("/:id/dump", get(course_dump))
        .route("/:id/bom", get(course_bom))
        .with_state(shared_state.clone());
    let course_routes = Router::new().nest("/course", course_routes);

    let set_routes = Router::new()
        .route("/list", get(set_list))
        .with_state(shared_state.clone());
    let set_routes = Router::new().nest("/set", set_routes);

    let api_routes = Router::new()
        .route("/buildable", post(buildable))
        .merge(course_routes)
        .merge(set_routes);

    let router = Router::new()
        .nest_service("/assets", serve_assets)
        .nest("/api", api_routes)
        .with_state(shared_state.clone())
        .route("/metrics", get(metrics).layer(build_prometheus_extension()))
        .layer(cors.clone())
        .fallback_service(serve_index);

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

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
