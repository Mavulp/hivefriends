use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, OptionalExtension};
use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::{api::auth::Authorize, api::error::Error, AppState};
use super::{DbImageMetadata, ImageMetadata};

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
                "SELECT \
                    key, \
                    uploader_key, \
                    uploaded_at, \
                    file_name, \
                    size_bytes, \
                    taken_at, \
                    location_latitude, \
                    location_longitude, \
                    camera_brand, \
                    camera_model, \
                    exposure_time, \
                    f_number, \
                    focal_length \
                FROM images \
                WHERE key = ?1",
                params![&ckey],
                |row| Ok(from_row::<DbImageMetadata>(row).unwrap()),
            )
            .optional()
        })
        .await
        .unwrap()
        .context("Failed to query image metadata")?;

    if let Some(image_metadata) = result {
        Ok(Json(ImageMetadata::from_db(image_metadata)))
    } else {
        Err(Error::NotFound)
    }
}
