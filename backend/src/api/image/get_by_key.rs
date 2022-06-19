use anyhow::Context;
use axum::{extract::Path, Extension, Json};

use std::sync::Arc;

use super::Image;
use crate::{api::auth::Authorize, api::error::Error, AppState};

pub(super) async fn get(
    Path(key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Image>, Error> {
    let ckey = key.clone();
    let result = state
        .db
        .call(move |conn| super::select_image(&ckey, conn))
        .await
        .context("Failed to query image metadata")?;

    if let Some(image_metadata) = result {
        Ok(Json(Image::from_db(image_metadata)))
    } else {
        Err(Error::NotFound)
    }
}
