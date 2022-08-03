use anyhow::Context;
use axum::{
    routing::{delete, get, post},
    Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use crate::api::error::Error;

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

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Comment {
    pub id: i64,
    pub author: String,
    pub image_key: String,
    pub album_key: String,
    pub created_at: u64,
    pub text: String,
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
        "INSERT INTO comments (author, image_key, album_key, created_at, text) \
        VALUES (?1, ?2, ?3, ?4, ?5)",
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

pub fn get_all(conn: &Connection) -> Result<Vec<Comment>, Error> {
    let mut stmt = conn
        .prepare(
            "SELECT c.id, c.text, c.author, c.image_key, c.album_key, c.created_at \
                FROM comments c \
                INNER JOIN images i ON c.image_key = i.key",
        )
        .context("Failed to prepare statement for comment query")?;

    let comments = stmt
        .query_map(params![], |row| Ok(from_row(row).unwrap()))
        .context("Failed to query comments")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect comments")?;

    Ok(comments)
}
