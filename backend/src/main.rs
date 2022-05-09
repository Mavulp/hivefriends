#![allow(dead_code)]

use anyhow::Context;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, Router},
    Extension, Json,
};
use deadpool_sqlite::{Config, Pool, Runtime};
use rusqlite_migration::{Migrations, M};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::*;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use std::net::SocketAddr;
use std::sync::Arc;

pub struct AppState {
    pool: Pool,
}

pub mod api {
    pub mod album;
    pub mod image;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    LogTracer::init().expect("Unable to setup log tracer!");

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::new("INFO"))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    // TODO add logger here

    if let Err(e) = run().await {
        let err = e
            .chain()
            .skip(1)
            .fold(e.to_string(), |acc, cause| format!("{}: {}\n", acc, cause));
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let pool = setup_database("test.db").await?;

    let bind_addr: SocketAddr = std::env::var("BIND_ADDRESS")
        .context("BIND_ADDRESS not set")?
        .parse()
        .context("BIND_ADDRESS could not be parsed")?;

    let app = Router::new()
        .route("/api/image/", post(dummy_login))
        .route("/api/login", post(dummy_login))
        .nest("/api/album/", api::album::api_route())
        .layer(Extension(Arc::new(AppState { pool })));

    info!("listening on {}", bind_addr);
    axum::Server::try_bind(&bind_addr)?
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn setup_database(path: &str) -> anyhow::Result<Pool> {
    let cfg = Config::new(path);
    let pool = cfg.create_pool(Runtime::Tokio1)?;

    let migrations = Migrations::new(vec![M::up(include_str!("../migrations/001_initial.sql"))]);

    let conn = pool.get().await?;
    conn.interact(move |mut conn| migrations.to_latest(&mut conn))
        .await
        .unwrap()?;

    Ok(pool)
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {}

async fn dummy_login(_: Json<LoginRequest>) -> Result<Json<LoginResponse>, ApiError> {
    Err(ApiError::Placeholder)
}

#[derive(Debug)]
enum ApiError {
    Placeholder,
    NotFound,
    InternalError(anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        eprintln!("{:#?}", self);
        let (status, error) = match self {
            ApiError::Placeholder => (StatusCode::NOT_IMPLEMENTED, "This is an error"),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "This is an error"),
            ApiError::InternalError(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "This is an error"),
        };

        let body = Json(json!({
            "error": error,
        }))
        .into_response();

        (status, body).into_response()
    }
}
