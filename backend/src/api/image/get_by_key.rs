use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;

use std::sync::Arc;

use crate::{api::auth::Authorize, api::error::Error, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ImageMetadata {
    key: String,
    uploader_key: String,
    created_at: i32,
}

pub(super) async fn get(
    Path(key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ImageMetadata>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;
    let ckey = key.clone();
    let result = conn
        .interact(move |conn| {
            conn.query_row(
                r"SELECT uploader_key, created_at FROM images WHERE key = ?1",
                params![&ckey],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?)),
            )
            .optional()
        })
        .await
        .unwrap()
        .context("Failed to query image metadata")?;

    if let Some((uploader_key, created_at)) = result {
        Ok(Json(ImageMetadata {
            key,
            uploader_key,
            created_at,
        }))
    } else {
        Err(Error::NotFound)
    }
}
