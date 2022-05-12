use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::Path, routing::get, Extension, Json, Router};
use rand::rngs::OsRng;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

pub fn api_route() -> Router {
    Router::new().route("/:id", get(get_user))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub met: Vec<String>,
    pub albums_uploaded: Vec<String>,
    pub created_at: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DbUser {
    id: i64,
    avatar_url: Option<String>,
    bio: Option<String>,
    created_at: i64,
}

async fn get_user(
    Path(username): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<User>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let cusername = username.clone();
    let result = conn
        .interact(move |conn| {
            conn.query_row(
                "SELECT id, avatar_url, bio, created_at \
                FROM users WHERE username = ?1",
                params![cusername],
                |row| Ok(from_row::<DbUser>(row).unwrap()),
            )
            .optional()
        })
        .await
        .unwrap();

    if let Some(db_user) = result.context("Failed to query database")? {
        let albums_uploaded = conn
            .interact(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT a.\"key\" FROM users u \
                INNER JOIN user_album_associations uaa ON uaa.user_id = u.id \
                INNER JOIN albums a ON a.id = uaa.album_id \
                WHERE u.id = ?1",
                )?;
                let album_key_iter = stmt.query_map(params![db_user.id], |row| {
                    Ok(from_row::<String>(row).unwrap())
                })?;

                album_key_iter.collect::<Result<Vec<_>, _>>()
            })
            .await
            .unwrap()
            .context("Failed to query database")?;


        let met = conn
            .interact(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT u2.username FROM users u1 \
                    INNER JOIN user_album_associations uaa ON uaa.user_id = u1.id \
                    INNER JOIN albums a ON a.id = uaa.album_id \
                    INNER JOIN user_album_associations uaa2 ON uaa2.album_id = a.id \
                    INNER JOIN users u2 ON uaa2.user_id = u2.id \
                    WHERE u1.id = ?1 \
                    AND u2.id != ?1",
                )?;
                let album_key_iter = stmt.query_map(params![db_user.id], |row| {
                    Ok(from_row::<String>(row).unwrap())
                })?;

                album_key_iter.collect::<Result<Vec<_>, _>>()
            })
            .await
            .unwrap()
            .context("Failed to query database")?;


        Ok(Json(User {
            username,
            avatar_url: db_user.avatar_url,
            bio: db_user.bio,
            met,
            albums_uploaded,
            created_at: db_user.created_at,
        }))
    } else {
        Err(Error::InvalidLogin)
    }
}

pub fn create_account(username: &str, password: &str, conn: &mut Connection) -> anyhow::Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let phc_string = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs() as u32;

    conn.execute(
        "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
        params![username, phc_string, now],
    )?;

    Ok(())
}

pub fn update_account(username: &str, password: &str, conn: &mut Connection) -> anyhow::Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let phc_string = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    conn.execute(
        r"
UPDATE users
SET password_hash = ?1
WHERE username = ?2",
        params![phc_string, username],
    )?;

    Ok(())
}
