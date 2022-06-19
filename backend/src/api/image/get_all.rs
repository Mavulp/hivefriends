use anyhow::Context;

use axum::{Extension, Json};

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::AppState;

use super::{DbImage, Image};
use std::sync::Arc;

use serde_rusqlite::from_row;

use rusqlite::params;

pub(super) async fn get_all_images(
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Image>>, Error> {
    state
        .db
        .call(move |conn| {
            let mut query = conn
                .prepare("SELECT * FROM images WHERE uploader=?")
                .context("Failed to prepare statement for images query")?;

            let dbdata = query
                .query_map(params![username], |row| {
                    Ok(Image::from_db(from_row::<DbImage>(row).unwrap()))
                })
                .context("Failed to query user images")?
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect user images")?;

            Ok(Json(dbdata))
        })
        .await
}
