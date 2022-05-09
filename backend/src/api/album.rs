use axum::{extract::Path, routing::get, Extension, Json, Router};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;

use std::sync::Arc;

use crate::{ApiError, AppState};

pub fn api_route() -> Router {
    Router::new()
        //.route("/", get(all_albums))
        .route("/:id", get(album_by_id))
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

async fn album_by_id(
    Path(id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Album>, ApiError> {
    match get_album_by_id(id, &state).await {
        Ok(Some(album)) => Ok(Json(album)),
        Ok(None) => Err(ApiError::NotFound),
        Err(e) => Err(ApiError::InternalError(e)),
    }
}

async fn get_album_by_id(album_id: String, state: &Arc<AppState>) -> anyhow::Result<Option<Album>> {
    let conn = state.pool.get().await?;

    conn.interact(move |mut conn| {
        let result = conn
            .query_row(
                r"SELECT id, created_at FROM album 
WHERE key=?1",
                params![album_id],
                |row| Ok((row.get::<_, i32>(0)?, row.get(1)?)),
            )
            .optional()?;

        if let Some((id, created_at)) = result {
            let mut stmt = conn.prepare(
                r"SELECT key, created_at FROM image 
INNER JOIN album_image_association ON image_id=id 
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
                key: album_id.to_string(),
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
