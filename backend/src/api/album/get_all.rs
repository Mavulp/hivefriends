use anyhow::Context;
use axum::{extract::Query, Extension, Json};
use rusqlite::{params, ToSql};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::{DbAlbum, Timeframe};

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

#[derive(Default, Deserialize)]
pub(super) struct AlbumFilters {
    #[serde(default)]
    #[serde(with = "comma_string")]
    user: Option<Vec<String>>,

    from: Option<u64>,
    to: Option<u64>,

    #[serde(default)]
    draft: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AlbumResponse {
    key: String,
    title: String,
    description: Option<String>,
    cover_key: String,
    author: String,
    draft: bool,
    timeframe: Timeframe,
    created_at: u64,
    tagged_users: Vec<String>,
}

pub(super) async fn get<D: SqliteDatabase>(
    Authorize(username): Authorize,
    Query(filter): Query<AlbumFilters>,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Vec<AlbumResponse>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let mut query = "SELECT \
                    key, \
                    title, \
                    description, \
                    cover_key, \
                    author, \
                    draft, \
                    timeframe_from, \
                    timeframe_to, \
                    created_at \
                FROM albums"
            .to_string();

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        apply_filters(&mut query, &mut params, filter, username);

        let mut stmt = conn
            .prepare(&format!(
                "{query} \
                ORDER BY \
                    created_at DESC"
            ))
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
                    "SELECT username FROM user_album_associations \
                    WHERE album_key = ?1",
                )
                .context("Failed to prepare statement for album query")?;
            let tagged_users = stmt
                .query_map(params![&db_album.key], |row| row.get(0))
                .context("Failed to query images")?
                .collect::<Result<Vec<String>, _>>()
                .context("Failed to collect tagged users")?;

            albums.push(AlbumResponse {
                key: db_album.key,
                title: db_album.title,
                description: db_album.description,
                cover_key: db_album.cover_key,
                author: db_album.author,
                draft: db_album.draft,
                timeframe: Timeframe {
                    from: db_album.timeframe_from,
                    to: db_album.timeframe_to,
                },
                created_at: db_album.created_at,
                tagged_users,
            })
        }
        Ok(Json(albums))
    })
    .await
}

fn apply_filters(
    query: &mut String,
    parameters: &mut Vec<Box<dyn ToSql>>,
    filters: AlbumFilters,
    username: String,
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

    filter_queries.push(draft_filter_query(parameters, filters.draft, username));

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn get_albums_tagged_users() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<Vec<String>> = conn
            .interact(move |conn| {
                let users = vec![insert_user("test", conn)?, insert_user("test2", conn)?];
                let user = users[0].clone();
                let image = insert_image(&user, conn).context("")?;
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;

                Ok(users)
            })
            .await;

        let users = result.unwrap();

        let result = get(
            Authorize("".into()),
            Query(AlbumFilters::default()),
            Extension(state),
        )
        .await;

        assert_matches!(result, Ok(Json(albums)) => {
            assert_matches!(&albums[..], [album] => {
                assert_eq!(album.tagged_users, users);
            })
        });
    }
}
