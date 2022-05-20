use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;

use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::api::image::{DbImageMetadata, ImageMetadata};
use crate::AppState;

use super::{DbAlbum, Timeframe};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AlbumResponse {
    key: String,
    title: String,
    description: Option<String>,
    cover_key: String,
    locations: Option<String>,
    uploader_key: String,
    draft: bool,
    timeframe: Timeframe,
    created_at: u64,
    images: Vec<ImageMetadata>,
}

pub(super) async fn get(
    Path(album_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<AlbumResponse>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let result = conn
            .query_row(
                "SELECT \
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
                FROM albums \
                WHERE key=?1",
                params![album_key],
                |row| Ok(from_row::<DbAlbum>(row).unwrap()),
            )
            .optional()
            .context("Failed to query albums")?;

        if let Some(db_album) = result {
            let mut stmt = conn
                .prepare(
                    "SELECT \
                        i.key, \
                        i.uploader_key, \
                        i.uploaded_at, \
                        i.file_name, \
                        i.size_bytes, \
                        i.taken_at, \
                        i.location_latitude, \
                        i.location_longitude, \
                        i.camera_brand, \
                        i.camera_model, \
                        i.exposure_time, \
                        i.f_number, \
                        i.focal_length \
                    FROM images i \
                    INNER JOIN album_image_associations aia ON aia.image_key=i.key \
                    WHERE aia.album_key=?1",
                )
                .context("Failed to prepare statement for image query")?;
            let image_iter = stmt
                .query_map(params![db_album.key], |row| {
                    Ok(ImageMetadata::from_db(
                        from_row::<DbImageMetadata>(row).unwrap(),
                    ))
                })
                .context("Failed to query images for album")?;

            let images = image_iter
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect album images")?;

            Ok(Json(AlbumResponse {
                key: album_key,
                title: db_album.title,
                description: db_album.description,
                cover_key: db_album.cover_key,
                locations: db_album.locations,
                uploader_key: db_album.uploader_key,
                draft: db_album.draft,
                timeframe: Timeframe {
                    from: db_album.timeframe_from,
                    to: db_album.timeframe_to,
                },
                created_at: db_album.created_at,
                images,
            }))
        } else {
            Err(Error::NotFound)
        }
    })
    .await
    .unwrap()
}
