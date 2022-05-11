use anyhow::Context;
use axum::{extract::Path, http::header::CONTENT_TYPE, routing::get, Extension, Router};
use headers::{HeaderMap, HeaderValue};
use rusqlite::{params, OptionalExtension};

use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Arc;

use crate::{api::error::Error, AppState};

pub fn api_route() -> Router {
    Router::new().route("/:key/:size", get(get_image))
}

async fn get_image(
    Path((key, size)): Path<(String, String)>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(HeaderMap<HeaderValue>, Vec<u8>), Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    if !["original", "medium", "tiny"].contains(&&size[..]) {
        return Err(Error::InvalidArguments(anyhow::format_err!(
            "Path parameter 'size' should be one of: 'original', 'small', 'tiny'"
        )));
    }

    let ckey = key.clone();
    conn.interact(move |conn| {
        conn.query_row(
            r"SELECT id FROM images WHERE key=?1",
            params![ckey],
            |row| row.get::<_, i32>(0),
        )
        .optional()
    })
    .await
    .unwrap()
    .context("Failed to query database")?
    .ok_or(Error::NotFound)?;

    let mut image_path = state.data_path.clone();
    image_path.push(key);
    image_path.push(format!("{}.png", size));

    let mut reader = BufReader::new(File::open(image_path).unwrap());

    let mut data = Vec::new();
    reader.read_to_end(&mut data).map_err(anyhow::Error::new)?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));

    Ok((headers, data))
}
