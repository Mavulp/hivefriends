use anyhow::Context;
use axum::{extract::Path, http::header::CONTENT_TYPE, routing::get, Extension, Router};
use headers::{HeaderMap, HeaderValue};
use rusqlite::{params, OptionalExtension};

use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Arc;

use crate::{api::error::Error, AppState};

pub fn api_route() -> Router {
    Router::new().route("/:key/:filename", get(get_image))
}

async fn get_image(
    Path((key, filename)): Path<(String, String)>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(HeaderMap<HeaderValue>, Vec<u8>), Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    if !["original.png", "medium.png", "tiny.png"].contains(&&filename[..]) {
        return Err(Error::InvalidArguments(anyhow::format_err!(
            "Path parameter 'filename' should be one of: 'original.png', 'small.png', 'tiny.png'"
        )));
    }

    let ckey = key.clone();
    conn.interact(move |conn| {
        conn.query_row(
            r"SELECT key FROM images WHERE key=?1",
            params![ckey],
            |row| row.get::<_, String>(0),
        )
        .optional()
    })
    .await
    .unwrap()
    .context("Failed to query database")?
    .ok_or(Error::NotFound)?;

    let mut image_path = state.data_path.clone();
    image_path.push(key);
    image_path.push(filename);

    let mut reader = BufReader::new(File::open(image_path).unwrap());

    let mut data = Vec::new();
    reader.read_to_end(&mut data).map_err(anyhow::Error::new)?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));

    Ok((headers, data))
}
