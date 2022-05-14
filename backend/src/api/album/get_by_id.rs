use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, OptionalExtension};

use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::error::Error;
use crate::AppState;

use super::{Album, DbAlbum, Image, Timeframe};

pub(super) async fn get(
    Path(album_key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Album>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let result = conn
            .query_row(
                "SELECT \
                    id, \
                    key, \
                    title, \
                    description, \
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
                    "SELECT i.key, i.uploader_key, i.created_at FROM images i \
                INNER JOIN album_image_associations aia ON aia.image_id=i.id \
                WHERE aia.album_id=?1",
                )
                .context("Failed to prepare statement for image query")?;
            let image_iter = stmt
                .query_map(params![db_album.id], |row| {
                    Ok(Image {
                        key: row.get(0)?,
                        uploader_key: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                })
                .context("Failed to query images for album")?;

            let images = image_iter
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect album images")?;
            Ok(Json(Album {
                key: album_key,
                title: db_album.title,
                description: db_album.description,
                locations: db_album.locations,
                uploader_key: db_album.uploader_key,
                draft: db_album.draft != 0,
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
