use anyhow::Context;
use axum::{
    extract::{rejection::JsonRejection},
    routing::{get, post},
    Extension, Json, Router,
};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(post_create_album))
        .route("/", get(get_albums))
        .route("/:key", get(get_album_by_key))
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeframe {
    from: Option<u64>,
    to: Option<u64>,
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
                "INSERT INTO album_image_associations (album_id, image_id) \
                SELECT ?1, id FROM images WHERE key = ?2",
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
    uploader_key: String,
    created_at: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Album {
    key: String,
    title: String,
    description: Option<String>,
    locations: Option<String>,
    timeframe: Timeframe,
    created_at: u64,
    images: Vec<Image>,
}

#[derive(Deserialize)]
struct DbAlbum {
    id: i64,
    key: String,
    title: String,
    description: Option<String>,
    locations: Option<String>,
    timeframe_from: Option<u64>,
    timeframe_to: Option<u64>,
    created_at: u64,
}


async fn get_albums(
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>) -> Result<Json<Vec<Album>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let mut stmt = conn
            .prepare(
                "SELECT \
                    id, \
                    key, \
                    title, \
                    description, \
                    locations, \
                    timeframe_from, \
                    timeframe_to, \
                    created_at \
                FROM albums",
            )
            .context("Failed to prepare statement for album query")?;
        let db_albums = stmt
            .query_map(params![], |row| Ok(from_row::<DbAlbum>(row).unwrap()))
            .context("Failed to query images")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect albums")?;

        let mut albums = Vec::new();
        for db_album in db_albums {
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
                .context("Failed to query images")?;

            let images = image_iter
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect images")?;

            albums.push(Album {
                key: db_album.key,
                title: db_album.title,
                description: db_album.description,
                locations: db_album.locations,
                timeframe: Timeframe {
                    from: db_album.timeframe_from,
                    to: db_album.timeframe_to,
                },
                created_at: db_album.created_at,
                images,
            })
        }
        Ok(Json(albums))
    })
    .await
    .unwrap()
}

/// Gets album information.
///
/// # Example response
/// ```json
/// {
/// }
/// ```

async fn get_album_by_key(
    album_key: String,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Album>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let result = conn
            .query_row(
                "SELECT id, title, description, locations, timeframe_from, timeframe_to, created_at \
                FROM albums \
                WHERE key=?1",
                params![album_key],
                |row| Ok(from_row::<DbAlbum>(row).unwrap()),
            )
            .optional().context("Failed to query albums")?;

        if let Some(db_album) = result {
            let mut stmt = conn.prepare(
                "SELECT i.key, i.uploader_key, i.created_at FROM images i \
                INNER JOIN album_image_associations aia ON aia.image_id=i.id \
                WHERE aia.album_id=?1",
            ).context("Failed to prepare statement for image query")?;
            let image_iter = stmt.query_map(params![db_album.id], |row| {
                Ok(Image {
                    key: row.get(0)?,
                    uploader_key: row.get(1)?,
                    created_at: row.get(2)?,
                })
            }).context("Failed to query images for album")?;

            let images = image_iter.collect::<Result<Vec<_>, _>>().context("Failed to collect album images")?;
            Ok(Json(Album {
                key: album_key,
                title: db_album.title,
                description: db_album.description,
                locations: db_album.locations,
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
