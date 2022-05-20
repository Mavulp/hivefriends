use anyhow::Context;
use axum::{extract::rejection::JsonRejection, Extension, Json};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error};
use crate::util::non_empty_str;
use crate::AppState;

use super::Timeframe;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateAlbumRequest {
    title: String,
    #[serde(default, deserialize_with = "non_empty_str")]
    description: Option<String>,
    cover_key: String,
    #[serde(default, deserialize_with = "non_empty_str")]
    locations: Option<String>,
    timeframe: Timeframe,
    image_keys: Vec<String>,
    #[serde(default)]
    draft: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateAlbumResponse {
    key: String,
}

pub(super) async fn post(
    request: Result<Json<CreateAlbumRequest>, JsonRejection>,
    Authorize(uploader_key): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<CreateAlbumResponse>, Error> {
    let Json(request) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    if !request.image_keys.contains(&request.cover_key) {
        return Err(Error::InvalidCoverKey);
    }

    let now = SystemTime::UNIX_EPOCH
        .elapsed()
        .context("Failed to get current time")?
        .as_secs();
    let key = blob_uuid::random_blob();

    let album_key = key.clone();
    conn.interact::<_, anyhow::Result<()>>(move |conn| {
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO albums ( \
                key, \
                title, \
                description, \
                cover_key, \
                locations, \
                uploader_key, \
                draft, \
                timeframe_from, \
                timeframe_to, \
                created_at \
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                album_key,
                request.title,
                request.description,
                request.cover_key,
                request.locations,
                uploader_key,
                request.draft as i64,
                request.timeframe.from,
                request.timeframe.to,
                now
            ],
        )?;

        for image_key in request.image_keys {
            tx.execute(
                "INSERT INTO album_image_associations (album_key, image_key) \
                SELECT ?1, key FROM images WHERE key = ?2",
                params![album_key, image_key],
            )?;
        }

        tx.commit()?;

        Ok(())
    })
    .await
    .unwrap()?;

    Ok(Json(CreateAlbumResponse { key }))
}
