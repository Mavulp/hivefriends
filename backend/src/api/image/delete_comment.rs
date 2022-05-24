use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;
use tracing::*;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Comment;

#[derive(Eq, PartialEq, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentRequest {
    id: u64,
}

pub(super) async fn delete<D: SqliteDatabase>(
    Path((image_key, comment_id)): Path<(String, i64)>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Comment>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        if !super::image_exists(&image_key, conn)? {
            return Err(Error::NotFound);
        }

        match super::get_comment(comment_id, conn)? {
            Some(comment) => {
                if comment.image_key != image_key {
                    return Err(Error::WrongImage);
                }

                if comment.author != user {
                    return Err(Error::Unathorized);
                }

                info!("Deleting comment {comment_id}");
                conn.execute("DELETE FROM comments WHERE id = ?1", params![comment_id])
                    .context("Failed to delete comment")?;

                Ok(Json(comment))
            }
            None => Err(Error::NotFound),
        }
    })
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::test::{insert_comment, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn delete_comment_invalid_image() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, i64)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let Comment { id, .. } = insert_comment(&user, &image, "text", conn)?;

                Ok((user, id))
            })
            .await;

        let (user, id) = result.unwrap();

        let result = delete(
            Path(("no_image".into(), id)),
            Authorize(user),
            Extension(state),
        )
        .await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn delete_comment_invalid_comment() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let _ = insert_comment(&user, &image, "text", conn)?;

                Ok((user, image))
            })
            .await;

        let (user, image) = result.unwrap();

        let result = delete(Path((image, -1)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn delete_comment_wrong_user() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String, i64)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let Comment { id, .. } = insert_comment(&user, &image, "text", conn)?;
                let user2 = insert_user("test2", conn)?;

                Ok((user2, image, id))
            })
            .await;

        let (user, image, comment) = result.unwrap();

        let result = delete(Path((image, comment)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::Unathorized));
    }

    #[tokio::test]
    async fn delete_comment_wrong_image() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String, i64)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let Comment { id, .. } = insert_comment(&user, &image, "text", conn)?;
                let image2 = insert_image(&user, conn)?;

                Ok((user, image2, id))
            })
            .await;

        let (user, image, comment) = result.unwrap();

        let result = delete(Path((image, comment)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::WrongImage));
    }

    #[tokio::test]
    async fn delete_comment() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String, Comment)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;
                let comment = insert_comment(&user, &image, "text", conn)?;

                Ok((user, image, comment))
            })
            .await;

        let (user, image, comment) = result.unwrap();

        let result = delete(Path((image, comment.id)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(deleted_comment)) => {
            assert_eq!(comment, deleted_comment);
        })
    }
}
