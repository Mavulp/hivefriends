use anyhow::Context;
use axum::{extract::rejection::JsonRejection, extract::Path, Extension, Json};
use rusqlite::{params, ToSql};
use serde::Deserialize;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error, image::image_exists, user::user_exists};
use crate::util::non_empty_str;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Timeframe;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutAlbumRequest {
    #[serde(default, deserialize_with = "non_empty_str")]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub cover_key: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub author: Option<String>,
    pub draft: Option<bool>,
    pub timeframe: Option<Timeframe>,
    pub created_at: Option<u64>,
    pub image_keys: Option<Vec<String>>,
    pub tagged_users: Option<Vec<String>>,
}

pub(super) async fn put<D: SqliteDatabase>(
    request: Result<Json<PutAlbumRequest>, JsonRejection>,
    Path(album_key): Path<String>,
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<&'static str>, Error> {
    let Json(request) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let tx = conn.transaction().context("Failed to create transaction")?;
        if !super::is_owner(&album_key, &username, &tx)? {
            return Err(Error::Unathorized);
        }

        if let Some(cover_key) = &request.cover_key {
            if !image_exists(cover_key, &tx)? {
                return Err(Error::InvalidKey);
            }
        }

        if let Some(image_keys) = &request.image_keys {
            tx.execute(
                "DELETE FROM album_image_associations WHERE album_key = ?",
                params![album_key],
            )
            .context("Failed to remove album image associations")?;

            for (idx, image_key) in (0_i64..).zip(image_keys.iter()) {
                if !image_exists(image_key, &tx)? {
                    return Err(Error::InvalidKey);
                }

                tx.execute(
                    "INSERT INTO album_image_associations (album_key, idx, image_key) \
                    SELECT ?1, ?2, key FROM images WHERE key = ?3",
                    params![album_key, idx, image_key],
                )
                .context("Failed to insert album image associations")?;
            }
        }

        if let Some(tagged_users) = &request.tagged_users {
            tx.execute(
                "DELETE FROM user_album_associations WHERE album_key = ?",
                params![album_key],
            )
            .context("Failed to remove album image associations")?;

            for tagged_user in tagged_users {
                if !user_exists(tagged_user, &tx)? {
                    return Err(Error::InvalidUsername);
                }

                tx.execute(
                    "INSERT INTO user_album_associations (album_key, username) VALUES (?1, ?2)",
                    params![album_key, tagged_user],
                )
                .context("Failed to insert album user associations")?;
            }
        }

        let update_str = request.album_update_str();
        if !update_str.is_empty() {
            let mut params = request.update_params();
            params.push(Box::new(album_key));
            params.push(Box::new(username));
            if let Err(rusqlite::Error::SqliteFailure(e, _)) = tx.query_row(
                &format!("UPDATE albums SET {update_str} WHERE key = ?"),
                rusqlite::params_from_iter(params.iter()),
                |_| Ok(()),
            ) {
                match e.code {
                    rusqlite::ErrorCode::ConstraintViolation => Err(Error::InvalidKey),
                    _ => panic!(),
                }
            } else {
                tx.commit().context("Failed to commit transaction")?;
                Ok(())
            }
        } else {
            tx.commit().context("Failed to commit transaction")?;
            Ok(())
        }
    })
    .await?;

    Ok(Json("Success"))
}

impl PutAlbumRequest {
    fn album_update_str(&self) -> String {
        let mut result = Vec::new();

        if self.title.is_some() {
            result.push("title = ?");
        }

        if self.description.is_some() {
            result.push("description = ?");
        }

        if self.cover_key.is_some() {
            result.push("cover_key = ?");
        }

        if self.draft.is_some() {
            result.push("draft = ?");
        }

        if self.timeframe.is_some() {
            result.push("timeframe_from = ?");
            result.push("timeframe_to = ?");
        }

        result.join(", ")
    }

    fn update_params(mut self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(title) = self.title.take() {
            params.push(Box::new(title))
        }

        if let Some(description) = self.description.take() {
            params.push(Box::new(description));
        }

        if let Some(cover_key) = self.cover_key.take() {
            params.push(Box::new(cover_key));
        }

        if let Some(draft) = self.draft.take() {
            params.push(Box::new(draft));
        }

        if let Some(timeframe) = self.timeframe.take() {
            params.push(Box::new(timeframe.from));
            params.push(Box::new(timeframe.to));
        }

        params
    }
}
