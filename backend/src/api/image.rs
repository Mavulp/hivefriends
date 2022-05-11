use axum::{
    extract::{
        multipart::MultipartRejection, rejection::ContentLengthLimitRejection, ContentLengthLimit,
        Multipart,
        Path,
    },
    routing::{post, get},
    Extension, Json, Router,
};
use rusqlite::{OptionalExtension, params};
use serde::Serialize;
use tokio::fs;
use anyhow::Context;

use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use crate::{api::auth::Authorize, api::error::Error, AppState};

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(post_upload_image))
        .route("/:id", get(get_image))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageMetadata {
    key: String,
    uploader_id: i32,
    created_at: i32,
}

async fn get_image(
    Path(key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
    ) -> Result<Json<ImageMetadata>, Error> {

    let conn = state.pool.get().await.context("Failed to get connection")?;
    let ckey = key.clone();
    let result = conn.interact(move |conn| {
        conn.query_row(
            r"SELECT uploader_id, created_at FROM images WHERE key = ?1",
            params![&ckey],
            |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)),
        )
        .optional()
    }).await.unwrap().context("Failed to query database")?;

    if let Some((uploader_id, created_at)) = result {
        Ok(Json(ImageMetadata {
            key,
            uploader_id,
            created_at,
        }))
    } else {
        Err(Error::NotFound)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageCreationResponse {
    key: String,
}

const MB: u64 = 1024 * 1024;

async fn post_upload_image(
    multipart: Result<
        ContentLengthLimit<Multipart, { 5 * MB }>,
        ContentLengthLimitRejection<MultipartRejection>,
    >,
    Authorize(user_id): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ImageCreationResponse>, Error> {
    let multipart = multipart?.0;

    match upload_image(multipart, user_id, &state).await {
        Ok(response) => Ok(Json(response)),
        Err(e) if e.is::<ImageCreationError>() => {
            match e.downcast_ref::<ImageCreationError>().unwrap() {
                ImageCreationError::NoImage => Err(Error::InvalidArguments(e)),
                ImageCreationError::ImageError(_) => Err(Error::InvalidArguments(e)),
            }
        }
        Err(e) => Err(Error::InternalError(e)),
    }
}

#[derive(Debug, thiserror::Error)]
enum ImageCreationError {
    #[error("Missing image data in multipart message")]
    NoImage,

    #[error("Failed to process image: {0}")]
    ImageError(#[from] image::ImageError),
}

async fn upload_image(
    mut multipart: Multipart,
    user_id: i64,
    state: &Arc<AppState>,
) -> anyhow::Result<ImageCreationResponse> {
    let field = multipart
        .next_field()
        .await?
        .ok_or(ImageCreationError::NoImage)?;

    let data = field.bytes().await?;

    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs() as u32;
    let key = blob_uuid::random_blob();
    store_image(state.data_path.clone(), &key, &data).await?;

    let image_key = key.clone();
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        conn.execute(
            r"INSERT INTO images (key, uploader_id, created_at) VALUES (?1, ?2, ?3)",
            params![&image_key, user_id, now],
        )
    })
    .await
    .unwrap()?;

    Ok(ImageCreationResponse { key })
}

async fn store_image(directory: PathBuf, key: &str, data: &[u8]) -> anyhow::Result<()> {
    let image = image::load_from_memory(data)?;
    let medium = image.thumbnail(512, 512);
    let tiny = medium.thumbnail(128, 128);

    let mut image_dir = directory;
    image_dir.push(key);

    fs::create_dir_all(&image_dir).await?;

    let mut buffer = Vec::new();

    for (image, name) in &[
        (image, "original.png"),
        (medium, "medium.png"),
        (tiny, "tiny.png"),
    ] {
        let mut path = image_dir.clone();
        path.push(name);

        buffer.clear();
        let mut cursor = Cursor::new(&mut buffer);
        image.write_to(&mut cursor, image::ImageOutputFormat::Png)?;
        fs::write(path, &buffer).await?;
    }

    Ok(())
}
