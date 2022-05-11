use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::rejection::JsonRejection,
    routing::{post, Router},
    Extension, Json,
};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{error::Error, user::User};
use crate::AppState;

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
    user: User,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DbUser {
    id: i64,
    password_hash: String,
    avatar_url: Option<String>,
    bio: Option<String>,
    created_at: i64,
}

async fn post_login(
    req: Result<Json<LoginRequest>, JsonRejection>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<LoginResponse>, Error> {
    let Json(req) = req?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let username = req.username.clone();
    let result = conn
        .interact(move |conn| {
            conn.query_row(
                "SELECT id, password_hash, avatar_url, bio, created_at \
                FROM users WHERE username=?1",
                params![username],
                |row| Ok(from_row::<DbUser>(row).expect("DbUser is for post_login")),
            )
            .optional()
        })
        .await
        .unwrap();

    if let Some(db_user) = result.context("Failed to query database")? {
        let argon2 = Argon2::default();
        let parsed_hash =
            PasswordHash::new(&db_user.password_hash).context("Failed creating hash")?;

        if argon2
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
            let bearer_token = generate_token();
            let token = bearer_token.clone();

            let conn = state
                .pool
                .get()
                .await
                .context("Failed getting DB connection")?;
            conn.interact(move |conn| {
                conn.execute(
                    "INSERT INTO auth_sessions (user_id, token, created_at) \
                    VALUES (?1, ?2, ?3)",
                    params![db_user.id, token, now],
                )
            })
            .await
            .unwrap()
            .context("Failed inserting token into DB")?;

            Ok(Json(LoginResponse {
                bearer_token,
                user: User {
                    username: req.username,
                    avatar_url: db_user.avatar_url,
                    bio: db_user.bio,
                    met: Vec::new(),
                    albums_uploaded: Vec::new(),
                    created_at: db_user.created_at,
                },
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
