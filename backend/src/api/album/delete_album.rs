use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;
use tracing::*;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::{AppState, DbInteractable, SqliteDatabase};

pub(super) async fn delete<D: SqliteDatabase>(
    Path(album_key): Path<String>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<()>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        if super::is_owner(&album_key, &user, conn)? {
            info!("Deleting album {album_key}");
            conn.execute("DELETE FROM albums WHERE key = ?1", params![album_key])
                .context("Failed to delete album")?;

            Ok(Json(()))
        } else {
            Err(Error::Unathorized)
        }
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
    async fn delete_album_invalid_album() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<String> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;

                Ok(user)
            })
            .await;

        let user = result.unwrap();

        let result = delete(Path(String::new()), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn delete_album_wrong_user() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;
                let user2 = insert_user("test2", conn)?;

                Ok((user2, album))
            })
            .await;

        let (user2, key) = result.unwrap();

        let result = delete(Path(key), Authorize(user2), Extension(state)).await;

        assert_matches!(result, Err(Error::Unathorized));
    }

    #[tokio::test]
    async fn delete_album_same_user() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;

                Ok((user, album))
            })
            .await;

        let (user, album_key) = result.unwrap();

        let result = delete(Path(album_key.clone()), Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(result)) => {
            assert_eq!((), result);
        })
    }
}
