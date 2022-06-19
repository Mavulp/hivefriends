use anyhow::Context;
use axum::{
    routing::{delete, get, post},
    Router,
};
use rusqlite::{params, Connection};

mod create_comment;
mod delete_comment;
mod get_all_comments;
mod get_shared_comments;

pub fn api_route() -> Router {
    Router::new()
        .route("/:album/:image", get(get_all_comments::get))
        .route("/:album/:image", post(create_comment::post))
        .route("/:album", delete(delete_comment::delete))
}

pub fn public_api_route() -> Router {
    Router::new().route("/:album/:image/:token", get(get_shared_comments::get))
}

#[derive(Eq, PartialEq, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: i64,
    author: String,
    image_key: String,
    album_key: String,
    created_at: u64,
    text: String,
}

pub fn insert_comment(
    author: String,
    text: String,
    image_key: String,
    album_key: String,
    created_at: u64,
    conn: &Connection,
) -> anyhow::Result<Comment> {
    conn.execute(
        "INSERT INTO comments (author, image_key, album_key, created_at, text) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&author, &image_key, &album_key, created_at, text],
    )
    .context("Failed to insert comment")?;

    let last_id = conn.last_insert_rowid();

    Ok(Comment {
        id: last_id,
        text,
        author,
        image_key,
        album_key,
        created_at,
    })
}

pub fn get_comment(id: i64, conn: &Connection) -> anyhow::Result<Option<Comment>> {
    let result = conn.query_row(
        "SELECT author, image_key, album_key, created_at, text FROM comments WHERE id = ?1",
        params![id],
        |row| {
            Ok(Comment {
                id,
                author: row.get(0)?,
                image_key: row.get(1)?,
                album_key: row.get(2)?,
                created_at: row.get(3)?,
                text: row.get(4)?,
            })
        },
    );

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        return Ok(None);
    }

    let comment = result.context("Failed to get comment")?;

    Ok(Some(comment))
}
