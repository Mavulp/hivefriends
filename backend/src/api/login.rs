use axum::{
    routing::{post, Router},
    Json,
};

use serde::{Deserialize, Serialize};
use crate::api::error::Error;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(post_login))
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    bearer_token: String
}

async fn post_login(_: Json<LoginRequest>) -> Result<Json<LoginResponse>, Error> {
    Err(Error::Placeholder)
}
