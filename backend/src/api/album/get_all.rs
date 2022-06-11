use anyhow::Context;
use axum::{extract::Query, Extension, Json};
use rusqlite::params;
use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::{apply_filters, AlbumFilters, AlbumMetadata, DbAlbum, Timeframe};

pub(super) async fn get<D: SqliteDatabase>(
    Authorize(username): Authorize,
    Query(filter): Query<AlbumFilters>,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Vec<AlbumMetadata>>, Error> {
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
            .context("Failed to query albums")?
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
                .context("Failed to query tagged users")?
                .collect::<Result<Vec<String>, _>>()
                .context("Failed to collect tagged users")?;

            albums.push(AlbumMetadata {
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
