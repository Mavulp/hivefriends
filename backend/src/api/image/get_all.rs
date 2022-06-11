use anyhow::Context;

use axum::{Extension, Json};

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::{DbImageMetadata, ImageMetadata};
use std::sync::Arc;

use serde_rusqlite::from_row;

use rusqlite::params;

pub(super) async fn get_all_images<D: SqliteDatabase>(
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Vec<ImageMetadata>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let mut query = conn
            .prepare("SELECT * FROM images WHERE uploader=?")
            .context("Failed to prepare statement for images query")?;

        let dbdata = query
            .query_map(params![username], |row| {
                Ok(ImageMetadata::from_db(
                    from_row::<DbImageMetadata>(row).unwrap(),
                ))
            })
            .context("Failed to query user images")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect user images")?;

        Ok(Json(dbdata))
    })
    .await
}

