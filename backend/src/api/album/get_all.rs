use anyhow::Context;
use axum::{extract::Query, Extension, Json};
use rusqlite::params;
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
}

pub(super) async fn get(
    Authorize(_): Authorize,
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
                    locations, \
                    timeframe_from, \
                    timeframe_to, \
                    created_at \
                FROM albums \
                "
        .to_string();

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        if let Some(users) = filter.user {
            query += &format!(
                "WHERE uploader_key IN ({}) \
                ",
                std::iter::repeat("?")
                    .take(users.len())
                    .collect::<Vec<_>>()
                    .join(",")
            );

            for user in users {
                params.push(Box::new(user));
            }
        }

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
                locations: db_album.locations,
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
