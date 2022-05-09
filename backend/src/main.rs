use anyhow::Context;
use axum::{
    Json,
    routing::{post, Router},
    http::StatusCode,
    response::{Response, IntoResponse},
};
use tracing::{error, info};
use serde::{Serialize, Deserialize};
use serde_json::json;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // TODO add logger here

    if let Err(e) = run().await {
        let err = e
            .chain()
            .skip(1)
            .fold(e.to_string(), |acc, cause| format!("{}: {}", acc, cause));
        error!("{}", err);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let bind_addr: SocketAddr = std::env::var("BIND_ADDRESS")
        .context("BIND_ADDRESS not set")?
        .parse()
        .context("BIND_ADDRESS could not be parsed")?;

    let app = Router::new().route("/api/login", post(dummy_login));

    info!("listening on {}", bind_addr);
    axum::Server::try_bind(&bind_addr)?
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
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

enum ApiError {
    Placeholder
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            ApiError::Placeholder => {
                (StatusCode::NOT_IMPLEMENTED, "This is an error")
            }
        };

        let body = Json(json!({
            "error": error,
        }))
        .into_response();

        (status, body).into_response()
    }
}
