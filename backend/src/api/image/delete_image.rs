use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;
use std::fs;
use tracing::*;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::AppState;

pub(super) async fn delete(
    Path(image_key): Path<String>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<()>, Error> {
    // We check later if the image key exists in the db but let's verify that we are not passing in
    // something bad just in case.
    blob_uuid::to_uuid(&image_key).map_err(|_| Error::NotFound)?;

    let mut image_path = state.data_path.clone();
    image_path.push(&image_key);

    state
        .db
        .call(move |conn| {
            if super::is_owner(&image_key, &user, conn)? {
                info!("Deleting image {image_key}");

                // TODO This could use await but not within the db closure.
                // Please be careful when messing with this. :)
                fs::remove_dir_all(image_path).context("Failed to remove image directory")?;

                conn.execute("DELETE FROM images WHERE key = ?1", params![image_key])
                    .context("Failed to delete image")?;

                Ok(Json(()))
            } else {
                Err(Error::Unathorized)
            }
        })
        .await
}
