use anyhow::Context;
use axum::{
    extract::rejection::JsonRejection,
    routing::{post, Router},
    Extension, Json,
};
use rand::seq::SliceRandom;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::{api::error::Error, AppState};

const AUTH_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz\
                       ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                       1234567890";
const AUTH_LENGTH: usize = 64;

pub fn api_route() -> Router {
    Router::new().route("/", post(post_login))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    bearer_token: String,
}

async fn post_login(
    req: Result<Json<LoginRequest>, JsonRejection>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<LoginResponse>, Error> {
    let Json(req) = req?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let result = conn
        .interact(move |conn| {
            conn.query_row(
                r"SELECT password_hash FROM users WHERE username=?1",
                params![req.username],
                |row| row.get::<_, String>(0),
            )
            .optional()
        })
        .await
        .unwrap();

    if let Some(hash) = result.context("Failed to query database")? {
        // FIXME WHERE MY HASHING AT
        if hash == req.password {
            let char_vec = AUTH_CHARSET.split("").collect::<Vec<&str>>();
            let mut rng = rand::thread_rng();

            let token =
                std::iter::repeat_with(|| char_vec.choose(&mut rng).expect("CHARSET not be empty"))
                    .take(AUTH_LENGTH)
                    .map(|s| *s)
                    .collect::<Vec<_>>()
                    .join("");

            // TODO store token in db

            Ok(Json(LoginResponse {
                bearer_token: token,
            }))
        } else {
            Err(Error::NotFound)
        }
    } else {
        Err(Error::NotFound)
    }
}
