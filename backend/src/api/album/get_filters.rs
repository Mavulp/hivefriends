use anyhow::Context;
use axum::{extract::Query, Extension, Json};
use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::{AlbumFilters, Timeframe};

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AvailableAlbumFilters {
    authors: Vec<String>,
    timeframes: Vec<Timeframe>,
    has_drafts: bool,
}

// This function returns available filters. If filters are sent in it returns the available filters
// that can be used alternatively.
//
// Example: The returned authors are all authors that can be found with all other filters applied.
// If the author is filtered on that is ignored for the returned author since otherwise it would
// only return one value. The same logic applies to all other filters.
pub(super) async fn get<D: SqliteDatabase>(
    Authorize(username): Authorize,
    Query(filter): Query<AlbumFilters>,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<AvailableAlbumFilters>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let mut authors: Vec<String> = get_filtered_values(
            conn,
            "author",
            AlbumFilters {
                author: None,
                ..filter
            },
            username.clone(),
        )?;

        authors.sort();
        authors.dedup();

        let mut timeframes: Vec<Timeframe> = get_filtered_values(
            conn,
            "timeframe_from AS \"from\", timeframe_to AS \"to\"",
            AlbumFilters {
                from: None,
                to: None,
                ..filter.clone()
            },
            username.clone(),
        )?;

        timeframes.sort();
        timeframes.dedup();

        let drafts: Vec<i64> = get_filtered_values(
            conn,
            "1",
            AlbumFilters {
                draft: true,
                ..filter
            },
            username,
        )?;

        Ok(Json(AvailableAlbumFilters {
            authors,
            timeframes,
            has_drafts: !drafts.is_empty(),
        }))
    })
    .await
}

fn get_filtered_values<T: DeserializeOwned>(
    conn: &rusqlite::Connection,
    column: &str,
    filters: AlbumFilters,
    username: String,
) -> Result<Vec<T>, Error> {
    let mut query = format!("SELECT {column} FROM albums");

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    crate::api::album::apply_filters(&mut query, &mut params, filters, username);

    let mut stmt = conn
        .prepare(&query)
        .context("Failed to prepare statement for draft query")?;

    let output: Vec<T> = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok(from_row::<T>(row).unwrap())
        })
        .context("Failed to query authors")?
        .try_collect()
        .context("Failed to collect authors")?;

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn get_all_filters() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let user = conn
            .interact(move |conn| {
                let user = insert_user("test", conn).unwrap();
                let image = insert_image(&user, conn).unwrap();
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        author: &user,
                        timeframe_from: Some(10),
                        timeframe_to: Some(100),
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        author: &user,
                        draft: true,
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                user
            })
            .await;

        let result = get(
            Authorize(user.clone()),
            Query(AlbumFilters::default()),
            Extension(state),
        )
        .await;

        dbg!(&result);

        assert_matches!(result, Ok(Json(filters)) => {
            assert_matches!(&filters.authors[..], [author] => {
                assert_eq!(&author[..], user);
            });

            assert_matches!(&filters.timeframes[0], timeframe => {
                assert_eq!(timeframe, &Timeframe {
                    from: Some(10),
                    to: Some(100),
                });
            });

            assert_eq!(filters.has_drafts, true);
        });
    }

    #[tokio::test]
    async fn get_draft_filters() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let user = conn
            .interact(move |conn| {
                let user = insert_user("test", conn).unwrap();
                let image = insert_image(&user, conn).unwrap();
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        author: &user,
                        timeframe_from: Some(10),
                        timeframe_to: Some(100),
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        author: &user,
                        draft: true,
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                user
            })
            .await;

        let result = get(
            Authorize(user.clone()),
            Query(AlbumFilters {
                draft: true,
                ..Default::default()
            }),
            Extension(state),
        )
        .await;

        assert_matches!(result, Ok(Json(filters)) => {
            assert_matches!(&filters.authors[..], [author] => {
                assert_eq!(&author[..], user);
            });

            assert_matches!(&filters.timeframes[0], timeframe => {
                assert_eq!(timeframe, &Timeframe {
                    from: None,
                    to: None,
                });
            });

            assert_eq!(filters.has_drafts, true);
        });
    }
}
