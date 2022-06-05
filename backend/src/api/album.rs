use anyhow::Context;
use axum::{
    routing::{get, post, put},
    Router,
};
use rusqlite::{params, Connection, ToSql};
use serde::{Deserialize, Serialize};
use serde_rusqlite::to_params_named;

use crate::api::error::Error;
use crate::util::comma_string;
use crate::FileDb;

use super::{image, user};

mod create;
mod create_share_token;
mod get_all;
mod get_by_key;
mod get_by_share_token;
mod get_filters;
mod update;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(create::post::<FileDb>))
        .route("/", get(get_all::get::<FileDb>))
        .route("/filters", get(get_filters::get::<FileDb>))
        .route("/:key", get(get_by_key::get::<FileDb>))
        .route("/:key", put(update::put::<FileDb>))
}

pub fn public_api_route() -> Router {
    Router::new()
        .route("/:album", post(create_share_token::post::<FileDb>))
        .route("/:album/:token", get(get_by_share_token::get::<FileDb>))
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Timeframe {
    from: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct DbAlbum {
    key: String,
    title: String,
    description: Option<String>,
    cover_key: String,
    author: String,
    draft: bool,
    timeframe_from: Option<i64>,
    timeframe_to: Option<i64>,
    created_at: u64,
}

#[derive(Default)]
pub struct InsertAlbum<'a> {
    pub key: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub cover_key: &'a str,
    pub author: &'a str,
    pub draft: bool,
    pub timeframe_from: Option<i64>,
    pub timeframe_to: Option<i64>,
    pub created_at: u64,
    pub image_keys: &'a [String],
    pub tagged_users: &'a [String],
}

#[derive(Default, Serialize)]
pub struct InsertShareToken<'a> {
    pub share_token: &'a str,
    pub album_key: &'a str,
    pub created_by: &'a str,
    pub created_at: u64,
}

pub fn is_owner(album_key: &str, user: &str, conn: &Connection) -> anyhow::Result<bool> {
    let result = conn.query_row(
        "SELECT author FROM albums WHERE key = ?1",
        params![album_key],
        |row| row.get::<_, String>(0),
    );

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        Ok(false)
    } else {
        let author = result?;

        Ok(author == user)
    }
}

pub fn insert_album(album: InsertAlbum, conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO albums ( \
                key, \
                title, \
                description, \
                cover_key, \
                author, \
                draft, \
                timeframe_from, \
                timeframe_to, \
                created_at \
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            album.key,
            album.title,
            album.description,
            album.cover_key,
            album.author,
            album.draft as i64,
            album.timeframe_from,
            album.timeframe_to,
            album.created_at
        ],
    )
    .context("Failed to insert album")?;

    for (idx, image_key) in (0..).zip(album.image_keys.iter()) {
        if !image::image_exists(image_key, conn)? {
            return Err(Error::InvalidKey);
        }

        conn.execute(
            "INSERT INTO album_image_associations (album_key, idx, image_key) \
            SELECT ?1, ?2, key FROM images WHERE key = ?3",
            params![album.key, idx, image_key],
        )
        .context("Failed to insert album image associations")?;
    }

    for user in album.tagged_users {
        if !user::user_exists(user, conn)? {
            return Err(Error::InvalidUsername);
        }

        conn.execute(
            "INSERT INTO user_album_associations (username, album_key) \
            VALUES (?1, ?2)",
            params![user, album.key],
        )
        .context("Failed to insert user album associations")?;
    }

    Ok(())
}

pub fn insert_share_token(rows: InsertShareToken, conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO album_share_tokens ( \
                share_token, \
                album_key, \
                created_by, \
                created_at \
            ) VALUES (
                :share_token,
                :album_key,
                :created_by,
                :created_at
            )",
        to_params_named(&rows).unwrap().to_slice().as_slice(),
    )
    .context("Failed to insert share token")?;

    Ok(())
}

#[derive(Clone, Default, Deserialize)]
pub(super) struct AlbumFilters {
    #[serde(default, deserialize_with = "comma_string")]
    author: Option<Vec<String>>,

    from: Option<u64>,
    to: Option<u64>,

    #[serde(default)]
    draft: bool,
}

fn apply_filters(
    query: &mut String,
    parameters: &mut Vec<Box<dyn ToSql>>,
    filters: AlbumFilters,
    username: String,
) {
    let mut filter_queries = Vec::new();

    if let Some(authors) = filters.author {
        filter_queries.push(author_filter_query(parameters, authors));
    }

    if let Some(from) = filters.from {
        filter_queries.push(from_filter_query(parameters, from));
    }

    if let Some(to) = filters.to {
        filter_queries.push(to_filter_query(parameters, to));
    }

    filter_queries.push(draft_filter_query(parameters, filters.draft, username));

    if !filter_queries.is_empty() {
        query.push_str(&format!(" WHERE {}", filter_queries.join(" AND ")));
    }
}

fn author_filter_query(parameters: &mut Vec<Box<dyn ToSql>>, authors: Vec<String>) -> String {
    let len = authors.len();

    for author in authors {
        parameters.push(Box::new(author));
    }

    format!(
        "author IN ({}) ",
        std::iter::repeat("?")
            .take(len)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn from_filter_query(parameters: &mut Vec<Box<dyn ToSql>>, from: u64) -> String {
    parameters.push(Box::new(from));
    let p = parameters.len();

    format!("(timeframe_from >= ?{p} OR timeframe_to >= ?{p})")
}

fn to_filter_query(parameters: &mut Vec<Box<dyn ToSql>>, to: u64) -> String {
    parameters.push(Box::new(to));
    let p = parameters.len();

    format!("(timeframe_to <= ?{p} OR timeframe_from <= ?{p})")
}

fn draft_filter_query(
    parameters: &mut Vec<Box<dyn ToSql>>,
    draft: bool,
    username: String,
) -> String {
    if draft {
        parameters.push(Box::new(username));
        let p = parameters.len();

        format!("(author = ?{p} AND draft = true)")
    } else {
        String::from("draft = false")
    }
}
