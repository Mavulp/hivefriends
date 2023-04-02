use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;
use tracing::*;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::AppState;

pub(super) async fn delete(
    Path(album_key): Path<String>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<()>, Error> {
    state
        .db
        .call(move |conn| {
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

        let user = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                user
            })
            .await;

        let result = delete(Path(String::new()), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn delete_album_wrong_user() {
        let state = AppState::in_memory_db().await;

        let (user2, album) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );
                let user2 = insert_user("test2", conn);

                (user2, album)
            })
            .await;

        let result = delete(Path(album), Authorize(user2), Extension(state)).await;

        assert_matches!(result, Err(Error::Unathorized));
    }

    #[tokio::test]
    async fn delete_album_same_user() {
        let state = AppState::in_memory_db().await;

        let (user, album) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                (user, album)
            })
            .await;

        let result = delete(Path(album.clone()), Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(_)) => {
        })
    }
}
