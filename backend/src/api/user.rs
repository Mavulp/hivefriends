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
    Router::new()
        .route("/", get(get_users))
        .route("/:key", get(get_user_by_key))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub key: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub met: Vec<String>,
    pub albums_uploaded: Vec<String>,
    pub created_at: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DbUser {
    key: String,
    username: String,
    avatar_url: Option<String>,
    bio: Option<String>,
    created_at: i64,
}

async fn get_users(
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let query = "SELECT \
                key, \
                username, \
                avatar_url, \
                bio, \
                created_at \
                FROM users"
            .to_string();

        let params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        let mut stmt = conn
            .prepare(&query)
            .context("Failed to prepare statement for album query")?;
        let db_users = stmt
            .query_map(rusqlite::params_from_iter(params.iter()), |row| {
                Ok(from_row::<DbUser>(row).unwrap())
            })
            .context("Failed to query images")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect albums")?;

        let mut users = Vec::new();
        for db_user in db_users {
            let ckey = db_user.key.clone();
            let mut stmt = conn
                .prepare(
                    "SELECT a.\"key\" FROM users u \
                    INNER JOIN user_album_associations uaa ON uaa.user_key = u.key \
                    INNER JOIN albums a ON a.id = uaa.album_id \
                    WHERE u.key = ?1",
                )
                .context("Failed to prepare user albums query")?;
            let album_key_iter = stmt
                .query_map(params![ckey], |row| Ok(from_row::<String>(row).unwrap()))
                .context("Failed to query user albums")?;

            let albums_uploaded = album_key_iter
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect albums uploaded")?;

            let ckey = db_user.key.clone();
            let mut stmt = conn
                .prepare(
                    "SELECT u2.key FROM users u1 \
                    INNER JOIN user_album_associations uaa ON uaa.user_key = u1.key \
                    INNER JOIN albums a ON a.id = uaa.album_id \
                    INNER JOIN user_album_associations uaa2 ON uaa2.album_id = a.id \
                    INNER JOIN users u2 ON uaa2.user_key = u2.key \
                    WHERE u1.key = ?1 \
                    AND u2.key != ?1",
                )
                .context("Failed to prepare met users query")?;
            let met_key_iter = stmt
                .query_map(params![ckey], |row| Ok(from_row::<String>(row).unwrap()))
                .context("Failed to query met users")?;

            let met = met_key_iter
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect met users")?;

            users.push(User {
                key: db_user.key,
                username: db_user.username,
                avatar_url: db_user.avatar_url,
                bio: db_user.bio,
                met,
                albums_uploaded,
                created_at: db_user.created_at,
            });
        }
        Ok(Json(users))
    })
    .await
    .unwrap()
}

async fn get_user_by_key(
    Path(user_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<User>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let result = conn
        .interact(move |conn| {
            conn.query_row(
                "SELECT key, username, avatar_url, bio, created_at \
                FROM users WHERE key = ?1",
                params![user_key],
                |row| Ok(from_row::<DbUser>(row).unwrap()),
            )
            .optional()
        })
        .await
        .unwrap()
        .context("Failed to query users")?;

    if let Some(db_user) = result {
        let ckey = db_user.key.clone();
        let albums_uploaded = conn
            .interact(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT a.\"key\" FROM users u \
                    INNER JOIN user_album_associations uaa ON uaa.user_key = u.key \
                    INNER JOIN albums a ON a.id = uaa.album_id \
                    WHERE u.key = ?1",
                )?;
                let album_key_iter =
                    stmt.query_map(params![ckey], |row| Ok(from_row::<String>(row).unwrap()))?;

                album_key_iter.collect::<Result<Vec<_>, _>>()
            })
            .await
            .unwrap()
            .context("Failed to query albums uploaded")?;

        let ckey = db_user.key.clone();
        let met = conn
            .interact(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT u2.key FROM users u1 \
                    INNER JOIN user_album_associations uaa ON uaa.user_key = u1.key \
                    INNER JOIN albums a ON a.id = uaa.album_id \
                    INNER JOIN user_album_associations uaa2 ON uaa2.album_id = a.id \
                    INNER JOIN users u2 ON uaa2.user_key = u2.key \
                    WHERE u1.key = ?1 \
                    AND u2.key != ?1",
                )?;
                let album_key_iter =
                    stmt.query_map(params![ckey], |row| Ok(from_row::<String>(row).unwrap()))?;

                album_key_iter.collect::<Result<Vec<_>, _>>()
            })
            .await
            .unwrap()
            .context("Failed to query met users")?;

        Ok(Json(User {
            key: db_user.key,
            username: db_user.username,
            avatar_url: db_user.avatar_url,
            bio: db_user.bio,
            met,
            albums_uploaded,
            created_at: db_user.created_at,
        }))
    } else {
        Err(Error::NotFound)
    }
}

pub fn create_account(username: &str, password: &str, conn: &mut Connection) -> anyhow::Result<()> {
    let key = blob_uuid::random_blob();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let phc_string = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs();

    conn.execute(
        "INSERT INTO users (key, username, password_hash, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![key, username, phc_string, now],
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
        "UPDATE users \
        SET password_hash = ?1 \
        WHERE username = ?2",
        params![phc_string, username],
    )?;

    Ok(())
}
