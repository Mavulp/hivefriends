use axum::{
    extract::{
        multipart::MultipartRejection, rejection::ContentLengthLimitRejection, ContentLengthLimit,
        Multipart,
    },
    Extension, Json,
};
use rusqlite::params;
use serde::Serialize;
use tokio::fs;

use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use crate::{api::auth::Authorize, api::error::Error, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ImageCreationResponse {
    key: String,
}

const MB: u64 = 1024 * 1024;

pub(super) async fn post(
    multipart: Result<
        ContentLengthLimit<Multipart, { 10 * MB }>,
        ContentLengthLimitRejection<MultipartRejection>,
    >,
    Authorize(user_key): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ImageCreationResponse>, Error> {
    let multipart = multipart?.0;

    match upload_image(multipart, user_key, &state).await {
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
    user_key: String,
    state: &Arc<AppState>,
) -> anyhow::Result<ImageCreationResponse> {
    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs();

    let field = multipart
        .next_field()
        .await?
        .ok_or(ImageCreationError::NoImage)?;

    let data = field.bytes().await?;

    let key = blob_uuid::random_blob();
    store_image(state.data_path.clone(), &key, &data).await?;

    let image_key = key.clone();
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        conn.execute(
            r"INSERT INTO images (key, uploader_key, created_at) VALUES (?1, ?2, ?3)",
            params![&image_key, user_key, now],
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
