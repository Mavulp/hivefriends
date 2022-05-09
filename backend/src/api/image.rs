use axum::{extract::Path, routing::post, Extension, Json, Router};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;

use std::sync::Arc;

use crate::{ApiError, AppState};

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(post_upload_image))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageCreationResult {
    key: String,
}

async fn post_upload_image(
    Path(id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ImageCreationResult>, ApiError> {
    match upload_image(id, &state).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(ApiError::InternalError(e)),
    }
}

async fn upload_image(album_id: String, state: &Arc<AppState>) -> anyhow::Result<ImageCreationResult> {
    todo!()
}
