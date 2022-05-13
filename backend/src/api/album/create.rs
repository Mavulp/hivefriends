use axum::{extract::rejection::JsonRejection, Extension, Json};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

use super::Timeframe;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateAlbumRequest {
    title: String,
    description: Option<String>,
    locations: Option<String>,
    timeframe: Timeframe,
    image_keys: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateAlbumResponse {
    key: String,
}

pub(super) async fn post(
    request: Result<Json<CreateAlbumRequest>, JsonRejection>,
    Authorize(uploader_key): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<CreateAlbumResponse>, Error> {
    let Json(request) = request?;

    match create_album(request, uploader_key, &state).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::InternalError(e)),
    }
}

async fn create_album(
    request: CreateAlbumRequest,
    uploader_key: String,
    state: &Arc<AppState>,
) -> anyhow::Result<CreateAlbumResponse> {
    let conn = state.pool.get().await?;

    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs() as u32;
    let key = blob_uuid::random_blob();

    let album_key = key.clone();
    conn.interact::<_, anyhow::Result<()>>(move |conn| {
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO albums \
                (key, title, description, locations, uploader_key, timeframe_from, timeframe_to, created_at) \
            VALUES \
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![album_key, request.title, request.description, request.locations, uploader_key, request.timeframe.from, request.timeframe.to, now])?;
        let album_id = tx.last_insert_rowid();

        for image_key in request.image_keys {
            tx.execute(
                "INSERT INTO album_image_associations (album_id, image_id) \
                SELECT ?1, id FROM images WHERE key = ?2",
                params![album_id, image_key])?;
        }

        tx.commit()?;

        Ok(())
    }).await.unwrap()?;

    Ok(CreateAlbumResponse { key })
}
