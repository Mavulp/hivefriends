use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::params;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::api::image;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Comment;

pub(super) async fn get<D: SqliteDatabase>(
    Path((album_key, image_key)): Path<(String, String)>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Vec<Comment>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        if !image::image_exists(&image_key, conn)? {
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
                    album_key: album_key.clone(),
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
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_comment, insert_image, insert_user};
    use assert_matches::assert_matches;
    use test_case::test_case;

    #[test_case("", "")]
    #[test_case("", "foo")]
    #[test_case("bar", "")]
    #[tokio::test]
    async fn get_invalid_image(album_key: &'static str, image_key: &'static str) {
        let state = AppState::in_memory_db().await;

        let result = get(
            Path((album_key.into(), image_key.into())),
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

        let result: anyhow::Result<(String, String)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn).context("")?;
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;
                insert_comment(&user, &image, &album, "foo", conn)?;
                insert_comment(&user, &image, &album, "bar", conn)?;

                Ok((album, image))
            })
            .await;

        let (album, image) = result.unwrap();

        let result = get(Path((album, image)), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(comments)) => {
            let comments = comments.iter().map(|c| c.text.as_str()).collect::<Vec<_>>();
            assert_matches!(comments[..], ["foo", "bar"]);
        });
    }

    #[tokio::test]
    async fn get_comments_multiple_images() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String)> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let key = insert_image(&user, conn).context("")?;
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &key,
                        image_keys: &[key.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;
                insert_comment(&user, &key, &album, "foo", conn)?;
                insert_comment(&user, &key, &album, "bar", conn)?;
                let key2 = insert_image(&user, conn).context("")?;
                let album2 = insert_album(
                    InsertAlbum {
                        cover_key: &key2,
                        image_keys: &[key2.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;
                insert_comment(&user, &key2, &album2, "quux", conn)?;

                Ok((album, key))
            })
            .await;

        let (album, image) = result.unwrap();

        let result = get(Path((album, image)), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(comments)) => {
            let comments = comments.iter().map(|c| c.text.as_str()).collect::<Vec<_>>();
            assert_matches!(comments[..], ["foo", "bar"]);
        });
    }
}
