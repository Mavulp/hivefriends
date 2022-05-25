use anyhow::Context;
use axum::{extract::rejection::JsonRejection, extract::Path, Extension, Json};

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::api::image;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Comment;

pub(super) async fn post<D: SqliteDatabase>(
    request: Result<Json<String>, JsonRejection>,
    Path((album_key, image_key)): Path<(String, String)>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Comment>, Error> {
    let Json(text) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();

    conn.interact(move |conn| {
        if !image::image_exists(&image_key, conn)? {
            return Err(Error::NotFound);
        }

        let comment = super::insert_comment(user, text, image_key, album_key, now, conn)?;

        Ok(Json(comment))
    })
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;
    use test_case::test_case;

    #[tokio::test]
    async fn create_comment_invalid_image() {
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

        let (user, album) = result.unwrap();

        let result = post(
            Ok(Json("".into())),
            Path((album, "no_image".into())),
            Authorize(user),
            Extension(state),
        )
        .await;

        assert_matches!(result, Err(Error::NotFound));
    }

    #[test_case("")]
    #[test_case("hello")]
    #[test_case("h̷͚͈͈͇̤̯̫̼͓̤̝͊͘t̸͉̰̿̈̎̀̂̔̓̀͛̕m̷̟̥͉͊͆̂̍͂͝l̶͉̇̂͊ ̵̨̻̲̼̬̮̬̽r̸̳̳̥͕̘͓̠̫̜̯̘̉͆̎̍̇͑̽̽̍̌̅e̸̘̮̫̪̥̹͈̳͙̣̮͈͙̚g̴̢̨̢̧̻͎̤͍͕͚̼͍̍͛̓̓̍͗͐͗͒̃͌̚̚̕͜͠ͅê̷͎̭̭̜̙͕x̵̢̢̧̧̖̞̱̳̬̞̻̊̉̔̄̓̐̽͒͋͘̕͝͝")]
    #[tokio::test]
    async fn create_comment(comment_text: &str) {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String, String)> = conn
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

                Ok((user, image, album))
            })
            .await;

        let (user, image, album) = result.unwrap();

        let result = post(
            Ok(Json(comment_text.into())),
            Path((album, image)),
            Authorize(user),
            Extension(state),
        )
        .await;

        assert_matches!(result, Ok(Json(Comment { text, .. })) => {
            assert_eq!(text, comment_text);
        });
    }
}
