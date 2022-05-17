use anyhow::Context;
use axum::{extract::Query, Extension, Json};
use rusqlite::{params, ToSql};
use serde::Deserialize;
use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

use super::{Album, DbAlbum, Image, Timeframe};

mod comma_string {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(Some(
                s.split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
            ));
        }

        Ok(None)
    }
}

#[derive(Deserialize)]
pub(super) struct AlbumFilters {
    #[serde(default)]
    #[serde(with = "comma_string")]
    user: Option<Vec<String>>,

    from: Option<u64>,
    to: Option<u64>,

    #[serde(default)]
    draft: bool,
}

pub(super) async fn get(
    Authorize(user_key): Authorize,
    Query(filter): Query<AlbumFilters>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Album>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let mut query = "SELECT \
                    id, \
                    key, \
                    title, \
                    description, \
                    cover_key, \
                    locations, \
                    uploader_key, \
                    draft, \
                    timeframe_from, \
                    timeframe_to, \
                    created_at \
                FROM albums"
            .to_string();

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        apply_filters(&mut query, &mut params, filter, user_key);

        let mut stmt = conn
            .prepare(&query)
            .context("Failed to prepare statement for album query")?;
        let db_albums = stmt
            .query_map(rusqlite::params_from_iter(params.iter()), |row| {
                Ok(from_row::<DbAlbum>(row).unwrap())
            })
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
                cover_key: db_album.cover_key,
                locations: db_album.locations,
                uploader_key: db_album.uploader_key,
                draft: db_album.draft != 0,
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

fn apply_filters(
    query: &mut String,
    parameters: &mut Vec<Box<dyn ToSql>>,
    filters: AlbumFilters,
    user_key: String,
) {
    let mut filter_queries = Vec::new();

    if let Some(users) = filters.user {
        filter_queries.push(user_filter_query(parameters, users));
    }

    if let Some(from) = filters.from {
        filter_queries.push(from_filter_query(parameters, from));
    }

    if let Some(to) = filters.to {
        filter_queries.push(to_filter_query(parameters, to));
    }

    filter_queries.push(draft_filter_query(parameters, filters.draft, user_key));

    if !filter_queries.is_empty() {
        query.push_str(&format!(" WHERE {}", filter_queries.join(" AND ")));
    }
}

fn user_filter_query(parameters: &mut Vec<Box<dyn ToSql>>, users: Vec<String>) -> String {
    let len = users.len();

    for user in users {
        parameters.push(Box::new(user));
    }

    format!(
        "uploader_key IN ({}) ",
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
    user_key: String,
) -> String {
    if draft {
        parameters.push(Box::new(user_key));
        let p = parameters.len();

        format!("(uploader_key = ?{p} OR draft = false)")
    } else {
        String::from("draft = false")
    }
}
