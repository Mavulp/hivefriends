use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Comment;

pub(super) async fn get<D: SqliteDatabase>(
    Path(image_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Vec<Comment>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        if !super::image_exists(&image_key, conn)? {
            return Err(Error::NotFound);
        }

        let mut stmt = conn
            .prepare(
                "SELECT c.id, c.text, c.author, c.created_at FROM comments c \
                INNER JOIN images i ON c.image_key=i.key \
                WHERE i.key=?1",
            )
            .context("Failed to prepare statement for comment query")?;

        let comment_iter = stmt
            .query_map(params![image_key], |row| {
                Ok(Comment {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    author: row.get(2)?,
                    image_key: image_key.clone(),
                    created_at: row.get(3)?,
                })
            })
            .context("Failed to query comments for image")?;

        let comments = comment_iter
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect image comments");

        if let Ok(comments) = comments {
            Ok(Json(comments))
        } else {
            Err(Error::NotFound)
        }
    })
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::test::{insert_comment, insert_image, insert_user};
    use assert_matches::assert_matches;
    use test_case::test_case;

    #[test_case("")]
    #[test_case("test")]
    #[tokio::test]
    async fn get_invalid_image(image_key: &'static str) {
        let state = AppState::in_memory_db().await;

        let result = get(
            Path(image_key.into()),
            Authorize("".into()),
            Extension(state),
        )
        .await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[tokio::test]
    async fn get_comments_single_image() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let key: anyhow::Result<String> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let key = insert_image(&user, conn).context("")?;
                insert_comment(&user, &key, "foo", conn)?;
                insert_comment(&user, &key, "bar", conn)?;

                Ok(key)
            })
            .await;

        let result = get(Path(key.unwrap()), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(comments)) => {
            let comments = comments.iter().map(|c| c.text.as_str()).collect::<Vec<_>>();
            assert_matches!(comments[..], ["foo", "bar"]);
        });
    }

    #[tokio::test]
    async fn get_comments_multiple_images() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let key: anyhow::Result<String> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let key = insert_image(&user, conn).context("")?;
                insert_comment(&user, &key, "foo", conn)?;
                insert_comment(&user, &key, "bar", conn)?;
                let key2 = insert_image(&user, conn).context("")?;
                insert_comment(&user, &key2, "quux", conn)?;

                Ok(key)
            })
            .await;

        let result = get(Path(key.unwrap()), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(comments)) => {
            let comments = comments.iter().map(|c| c.text.as_str()).collect::<Vec<_>>();
            assert_matches!(comments[..], ["foo", "bar"]);
        });
    }
}
