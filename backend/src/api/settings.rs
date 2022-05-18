use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::rejection::JsonRejection,
    routing::{get, put},
    Extension, Json, Router,
};
use rand::rngs::OsRng;
use rusqlite::{params, OptionalExtension, ToSql};
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
    pub private: Option<bool>,
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
    pub private: bool,
    pub color_theme: String,
}

async fn get_settings(
    Authorize(user_key): Authorize,
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
                    private, \
                    color_theme \
                FROM users WHERE key = ?1",
                params![user_key],
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
            private: Some(db_settings.private),
            color_theme: Some(db_settings.color_theme),
        }))
    } else {
        Err(Error::NotFound)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutSettingsRequest {
    pub password: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_key: Option<String>,
    pub banner_key: Option<String>,
    pub accent_color: Option<String>,
    pub featured_album_key: Option<String>,
    pub private: Option<bool>,
    pub color_theme: Option<String>,
}

async fn put_settings(
    request: Result<Json<PutSettingsRequest>, JsonRejection>,
    Authorize(user_key): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<&'static str>, Error> {
    let Json(request) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let update_str = request.update_str();
    if !update_str.is_empty() {
        if let Err(rusqlite::Error::SqliteFailure(e, _)) = conn
            .interact(move |conn| {
                let mut params = request.update_params();
                params.push(Box::new(user_key));
                conn.query_row(
                    &format!("UPDATE users SET {update_str} WHERE key = ?"),
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

impl PutSettingsRequest {
    fn update_str(&self) -> String {
        let mut result = Vec::new();

        if self.password.is_some() {
            result.push("password_hash = ?")
        }

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

        if self.private.is_some() {
            result.push("private = ?")
        }

        if self.color_theme.is_some() {
            result.push("color_theme = ?")
        }

        result.join(", ")
    }

    fn update_params(mut self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(password) = self.password.take() {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let phc_string = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            params.push(Box::new(phc_string))
        }

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

        if let Some(private) = self.private.take() {
            params.push(Box::new(private));
        }

        if let Some(color_theme) = self.color_theme.take() {
            params.push(Box::new(color_theme));
        }

        params
    }
}
