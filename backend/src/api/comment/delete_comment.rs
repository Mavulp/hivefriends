use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;
use tracing::*;

use std::sync::Arc;

use crate::api::album;
use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::AppState;

use super::Comment;

#[derive(Eq, PartialEq, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentRequest {
    id: u64,
}

pub(super) async fn delete(
    Path(comment_id): Path<i64>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Comment>, Error> {
    state
        .db
        .call(move |conn| match super::get_comment(comment_id, conn)? {
            Some(comment) => {
                if comment.author != user && !album::is_owner(&comment.album_key, &user, conn)? {
                    return Err(Error::Unathorized);
                }

                info!("Deleting comment {comment_id}");
                conn.execute("DELETE FROM comments WHERE id = ?1", params![comment_id])
                    .context("Failed to delete comment")?;

                Ok(Json(comment))
            }
            None => Err(Error::NotFound),
        })
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_comment, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn delete_comment_invalid_comment() {
        let state = AppState::in_memory_db().await;

        let user = state
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
                let _ = insert_comment(&user, &image, &album, "text", conn);

                user
            })
            .await;

        let result = delete(Path(-1), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn delete_comment_wrong_user() {
        let state = AppState::in_memory_db().await;

        let (user, comment) = state
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
                let Comment { id, .. } = insert_comment(&user, &image, &album, "text", conn);
                let user2 = insert_user("test2", conn);

                (user2, id)
            })
            .await;

        let result = delete(Path(comment), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::Unathorized));
    }

    #[tokio::test]
    async fn delete_comment_same_user() {
        let state = AppState::in_memory_db().await;

        let (user, comment) = state
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
                let comment = insert_comment(&user, &image, &album, "text", conn);

                (user, comment)
            })
            .await;

        let result = delete(Path(comment.id), Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(deleted_comment)) => {
            assert_eq!(comment, deleted_comment);
        })
    }

    #[tokio::test]
    async fn delete_comment_album_user() {
        let state = AppState::in_memory_db().await;

        let (user, comment) = state
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
                let comment = insert_comment(&user2, &image, &album, "text", conn);

                (user, comment)
            })
            .await;

        let result = delete(Path(comment.id), Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(deleted_comment)) => {
            assert_eq!(comment, deleted_comment);
        })
    }
}
