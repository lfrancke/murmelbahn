mod buildable;
mod course;
mod course_repo;
mod set;

use crate::buildable::buildable;
use crate::course::{course_bom, course_dump};
use crate::course_repo::CourseRepo;
use crate::set::set_list;
use axum::body::{Empty, Full};
use axum::extract::Path;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get};
use axum::{body, Extension, Json, Router};
use clap::Parser;
use include_dir::{include_dir, Dir};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use murmelbahn_lib::error::MurmelbahnError;
use murmelbahn_lib::set::SetRepo;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use axum::http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, info};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../gravitrax-frontend/dist");

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(env)]
    pub sets_directory: PathBuf,
}

pub struct AppState {
    course_repo: CourseRepo,
    sets_repo: SetRepo,
}

pub enum AppError {
    MurmelbahnLibError(MurmelbahnError),
    ZiplineAdded2019Unsupported,
    CourseError(course_repo::Error),
    JsonError(serde_json::Error),
}

// Makes it possible to use `?`
impl From<MurmelbahnError> for AppError {
    fn from(inner: MurmelbahnError) -> Self {
        AppError::MurmelbahnLibError(inner)
    }
}

impl From<course_repo::Error> for AppError {
    fn from(inner: course_repo::Error) -> Self {
        AppError::CourseError(inner)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(inner: serde_json::Error) -> Self {
        AppError::JsonError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MurmelbahnLibError(_murmelbahn_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::ZiplineAdded2019Unsupported => (
                StatusCode::BAD_REQUEST,
                "ZiplineAdded2019 data format is not currently supported",
            ),
            AppError::CourseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::JsonError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong serializing the response to JSON",
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let course_repo = CourseRepo::new(db);
    let mut sets_repo = SetRepo::new();
    sets_repo.read_directory(config.sets_directory)?;

    let shared_state = Arc::new(AppState {
        course_repo,
        sets_repo,
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(Any);

    let course_routes = Router::new()
        .route("/:id/dump", get(course_dump))
        .route("/:id/bom", get(course_bom))
        .with_state(shared_state.clone());

    let set_routes = Router::new()
        .route("/list", get(set_list))

        .with_state(shared_state.clone());

    let app = Router::new()
        .route("/metrics", get(metrics).layer(build_prometheus_extension()))
        .route("/buildable", get(buildable).post(buildable))
        .layer(cors.clone())
        .with_state(shared_state.clone())
        .nest("/course", course_routes)
        .nest("/set", set_routes)
        .route("/*path", get(static_path))
        .layer(cors)
        .route("/", get(|| async { Redirect::permanent("/index.html") }));

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
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

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
