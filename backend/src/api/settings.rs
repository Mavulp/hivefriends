use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::rejection::JsonRejection,
    routing::{get, put},
    Extension, Json, Router,
};
use rand::rngs::OsRng;
use rusqlite::{params, Connection, OptionalExtension, ToSql};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::util::non_empty_str;
use crate::AppState;

pub fn api_route() -> Router {
    Router::new()
        .route("/", get(get_settings))
        .route("/", put(put_settings))
        .route("/password", put(put_password))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default, deserialize_with = "non_empty_str")]
    pub display_name: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub bio: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub avatar_key: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub banner_key: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub accent_color: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub featured_album_key: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub color_theme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbSettings {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_key: Option<String>,
    pub banner_key: Option<String>,
    pub accent_color: Option<String>,
    pub featured_album_key: Option<String>,
    pub color_theme: String,
}

async fn get_settings(
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Settings>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let result = conn
        .interact(move |conn| {
            conn.query_row(
                "SELECT
                    display_name, \
                    bio, \
                    avatar_key, \
                    banner_key, \
                    accent_color, \
                    featured_album_key, \
                    color_theme \
                FROM users WHERE username = ?1",
                params![username],
                |row| Ok(from_row::<DbSettings>(row).unwrap()),
            )
            .optional()
        })
        .await
        .unwrap()
        .context("Failed to query settings")?;

    if let Some(db_settings) = result {
        Ok(Json(Settings {
            display_name: db_settings.display_name,
            bio: db_settings.bio,
            avatar_key: db_settings.avatar_key,
            banner_key: db_settings.banner_key,
            accent_color: db_settings.accent_color,
            featured_album_key: db_settings.featured_album_key,
            color_theme: Some(db_settings.color_theme),
        }))
    } else {
        Err(Error::NotFound)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutSettingsRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_key: Option<String>,
    pub banner_key: Option<String>,
    pub accent_color: Option<String>,
    pub featured_album_key: Option<String>,
    pub color_theme: Option<String>,
}

async fn put_settings(
    request: Result<Json<PutSettingsRequest>, JsonRejection>,
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<&'static str>, Error> {
    let Json(request) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let update_str = request.update_str();
    if !update_str.is_empty() {
        if let Err(rusqlite::Error::SqliteFailure(e, _)) = conn
            .interact(move |conn| {
                let mut params = request.update_params();
                params.push(Box::new(username));
                conn.query_row(
                    &format!("UPDATE users SET {update_str} WHERE username = ?"),
                    rusqlite::params_from_iter(params.iter()),
                    |row| Ok(from_row::<Settings>(row).unwrap()),
                )
                .optional()
            })
            .await
            .unwrap()
        {
            match e.code {
                rusqlite::ErrorCode::ConstraintViolation => return Err(Error::InvalidKey),
                _ => panic!(),
            }
        }
    }

    Ok(Json("Success"))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutPasswordRequest {
    pub old: String,
    pub new: String,
}

async fn put_password(
    request: Result<Json<PutPasswordRequest>, JsonRejection>,
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<&'static str>, Error> {
    let Json(request) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let cusername = username.clone();
    let result = conn
        .interact(move |conn| {
            conn.query_row(
                "SELECT password_hash \
                FROM users WHERE username=?1",
                params![cusername],
                |row| row.get::<_, String>(0),
            )
            .optional()
        })
        .await
        .unwrap()
        .context("Failed to query username")?;

    if let Some(password_hash) = result {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&password_hash).context("Failed creating hash")?;

        if argon2
            .verify_password(request.old.as_bytes(), &parsed_hash)
            .is_ok()
        {
            conn.interact(move |conn| set_password(&username, &request.new, conn))
                .await
                .unwrap()
                .context("Failed to set password")?;

            Ok(Json("Success"))
        } else {
            Err(Error::InvalidPassword)
        }
    } else {
        Err(Error::NotFound)
    }
}

fn set_password(username: &str, password: &str, conn: &Connection) -> Result<(), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let phc_string = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    conn.execute(
        "UPDATE users SET password_hash = ? WHERE username = ?",
        params![phc_string, username],
    )
    .context("Failed to update password hash")?;

    Ok(())
}

impl PutSettingsRequest {
    fn update_str(&self) -> String {
        let mut result = Vec::new();

        if self.display_name.is_some() {
            result.push("display_name = ?")
        }

        if self.bio.is_some() {
            result.push("bio = ?")
        }

        if self.avatar_key.is_some() {
            result.push("avatar_key = ?")
        }

        if self.banner_key.is_some() {
            result.push("banner_key = ?")
        }

        if self.accent_color.is_some() {
            result.push("accent_color = ?")
        }

        if self.featured_album_key.is_some() {
            result.push("featured_album_key = ?")
        }

        if self.color_theme.is_some() {
            result.push("color_theme = ?")
        }

        result.join(", ")
    }

    fn update_params(mut self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(display_name) = self.display_name.take() {
            params.push(Box::new(display_name))
        }

        if let Some(bio) = self.bio.take() {
            params.push(Box::new(bio));
        }

        if let Some(avatar_key) = self.avatar_key.take() {
            params.push(Box::new(avatar_key));
        }

        if let Some(banner_key) = self.banner_key.take() {
            params.push(Box::new(banner_key));
        }

        if let Some(accent_color) = self.accent_color.take() {
            params.push(Box::new(accent_color));
        }

        if let Some(featured_album_key) = self.featured_album_key.take() {
            params.push(Box::new(featured_album_key));
        }

        if let Some(color_theme) = self.color_theme.take() {
            params.push(Box::new(color_theme));
        }

        params
    }
}
