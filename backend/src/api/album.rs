use axum::{
    extract::{rejection::JsonRejection, Path},
    routing::{get, post},
    Extension, Json, Router,
};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::time::SystemTime;

use crate::AppState;
use crate::api::{error::Error, auth::Authorize};

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(post_create_album))
        .route("/:id", get(album_by_id))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbumRequest {
    title: String,
    description: Option<String>,
    locations: Option<String>,
    timeframe: Timeframe,
    image_keys: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeframe {
    from: Option<i64>,
    to: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbumResponse {
    key: String,
}

/// Creates a new album.
///
/// # Example request
/// ```json
/// {
///     "name": "My album!",
///     "imageKeys": [
///         "klaFLKF31",
///         "sia29mFKa",
///         "PqqQ3p1km"
///     ]
/// }
/// ```
///
/// # Example response
/// ```json
/// {
///     "key": "asdasfaa"
/// }
/// ```
async fn post_create_album(
    request: Result<Json<CreateAlbumRequest>, JsonRejection>,
    Authorize(uploader_key): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<CreateAlbumResponse>, Error> {
    let Json(request) = request?;

    match create_album(request, uploader_key, &state).await {
        Ok(response) => Ok(Json(response)),
        /*Err(e) if e.is::<CreateAlbumError>() => {
            match e.downcast_ref::<ImageCreationError>().unwrap() {
                ImageCreationError::NoImage => Err(Error::InvalidArguments(e)),
                ImageCreationError::ImageError(_) => Err(Error::InvalidArguments(e)),
            }
        }*/
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
                r"INSERT INTO album_image_associations (album_id, image_id) SELECT ?1, id FROM images WHERE key = ?2",
                params![album_id, image_key])?;
        }

        tx.commit()?;

        Ok(())
    }).await.unwrap()?;

    Ok(CreateAlbumResponse { key })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    key: String,
    created_at: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Album {
    key: String,
    created_at: u32,
    images: Vec<Image>,
}

/// Gets album information.
///
/// # Example response
/// ```json
/// {
/// }
/// ```
async fn album_by_id(
    Path(id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Album>, Error> {
    match get_album_by_id(id, &state).await {
        Ok(Some(album)) => Ok(Json(album)),
        Ok(None) => Err(Error::NotFound),
        Err(e) => Err(Error::InternalError(e)),
    }
}

async fn get_album_by_id(album_id: String, state: &Arc<AppState>) -> anyhow::Result<Option<Album>> {
    let conn = state.pool.get().await?;

    conn.interact(move |conn| {
        let result = conn
            .query_row(
                r"SELECT id, created_at FROM albums 
WHERE key=?1",
                params![album_id],
                |row| Ok((row.get::<_, i32>(0)?, row.get(1)?)),
            )
            .optional()?;

        if let Some((id, created_at)) = result {
            let mut stmt = conn.prepare(
                r"SELECT key, created_at FROM images 
INNER JOIN album_image_associations ON image_id=id 
WHERE album_id=?1",
            )?;
            let image_iter = stmt.query_map(params![id], |row| {
                Ok(Image {
                    key: row.get(0)?,
                    created_at: row.get(1)?,
                })
            })?;

            let images = image_iter.collect::<Result<Vec<_>, _>>()?;
            Ok(Some(Album {
                key: album_id,
                created_at,
                images,
            }))
        } else {
            Ok(None)
        }
    })
    .await
    .unwrap()
}
