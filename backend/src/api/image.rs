use axum::{
    extract::{
        multipart::MultipartRejection, rejection::ContentLengthLimitRejection, ContentLengthLimit,
        Multipart,
    },
    routing::post,
    Extension, Json, Router,
};
use rusqlite::params;
use serde::Serialize;
use tokio::fs;

use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use crate::{api::error::Error, api::auth::Authorize, AppState};

pub fn api_route() -> Router {
    Router::new().route("/", post(post_upload_image))
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
    store_image(&state.data_path, &key, &data).await?;

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

    Ok(ImageCreationResponse { key: key })
}

async fn store_image(directory: &PathBuf, key: &str, data: &[u8]) -> anyhow::Result<()> {
    let image = image::load_from_memory(data)?;
    let medium = image.thumbnail(512, 512);
    let tiny = medium.thumbnail(128, 128);

    let mut image_dir = directory.clone();
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
