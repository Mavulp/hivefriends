use anyhow::Context;
use axum::{extract::Path, Extension, Json};

use std::sync::Arc;

use super::ImageMetadata;
use crate::{api::auth::Authorize, api::error::Error, AppState};

pub(super) async fn get(
    Path(key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ImageMetadata>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;
    let ckey = key.clone();
    let result = conn
        .interact(move |conn| super::select_image(&ckey, conn))
        .await
        .unwrap()
        .context("Failed to query image metadata")?;

    if let Some(image_metadata) = result {
        Ok(Json(ImageMetadata::from_db(image_metadata)))
    } else {
        Err(Error::NotFound)
    }
}
