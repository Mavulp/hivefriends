use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::rejection::JsonRejection,
    routing::{post, Router},
    Extension, Json,
};
use rand::seq::SliceRandom;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;

use std::sync::Arc;
use std::time::SystemTime;

use crate::{api::error::Error, AppState};

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
                r"SELECT id, password_hash FROM users WHERE username=?1",
                params![req.username],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()
        })
        .await
        .unwrap();

    if let Some((id, hash)) = result.context("Failed to query database")? {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&hash).context("Failed creating hash")?;

        if argon2
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
            let bearer_token = generate_token();
            let token = bearer_token.clone();

            let conn = state.pool.get().await.context("Failed getting DB connection")?;
            conn.interact(move |conn| {
                conn.execute(
                    r"INSERT INTO auth_sessions (user_id, token, created_at) VALUES (?1, ?2, ?3)",
                    params![id, token, now])
            }).await.unwrap().context("Failed inserting token into DB")?;

            Ok(Json(LoginResponse {
                bearer_token,
            }))
        } else {
            Err(Error::InvalidLogin)
        }
    } else {
        Err(Error::InvalidLogin)
    }
}

fn generate_token() -> String {
    const AUTH_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz\
                           ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           1234567890";
    const AUTH_LENGTH: usize = 64;

    let char_vec = AUTH_CHARSET.split("").collect::<Vec<&str>>();

    std::iter::repeat_with(|| char_vec.choose(&mut OsRng).expect("CHARSET is not empty"))
        .take(AUTH_LENGTH)
        .copied()
        .collect::<Vec<_>>()
        .join("")
}
