use anyhow::Context;
use axum::{extract::rejection::JsonRejection, extract::Path, Extension, Json};

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::api::image;
use crate::{AppState, DbInteractable, SqliteDatabase};

use rusqlite::{params, OptionalExtension};

use super::Comment;

pub(super) async fn post<D: SqliteDatabase>(
    request: Result<Json<String>, JsonRejection>,
    Path((album_key, image_key)): Path<(String, String)>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Comment>, Error> {
    let Json(mut text) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();

    // Chek for possible aliases within the text
    // and return the modified string
    text = extract_alias(text, &state).await?;

    conn.interact(move |conn| {
        if !image::image_exists(&image_key, conn)? {
            return Err(Error::NotFound);
        }

        let comment = super::insert_comment(user, text, image_key, album_key, now, conn)?;

        Ok(Json(comment))
    })
    .await
}

async fn extract_alias<D: SqliteDatabase>(
    mut text: String,
    state: &Arc<AppState<D>>,
) -> anyhow::Result<String> {
    let aliases: Vec<String> = text
        .split(" ")
        .filter(|word| word.starts_with("!"))
        .map(|word| word[1..].to_string())
        .filter(|word| !word.is_empty())
        .collect();

    for alias in aliases {
        // For each alias, fetch the link
        //
        // If link is available, replace alias with link
        //
        // If no alias is available, skip
        let cloned = alias.clone();
        let conn = state.pool.get().await.context("Failed to get connection")?;

        let alias_link = conn
            .interact(move |conn| {
                conn.query_row(
                    r"SELECT content FROM aliases WHERE name=?1",
                    params![&cloned],
                    |row| Ok(serde_rusqlite::from_row::<String>(row).unwrap()),
                )
                .optional()
            })
            .await
            .unwrap();

        if let Some(alias_link) = alias_link {
            text = text.replace(&format!("!{alias}"), &alias_link);
        }
    }

    Ok(text)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_alias, insert_image, insert_user};
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

    #[test_case("LMAO !pogu", "LMAO __pog__u__url__", vec![("pogu","__pog__u__url__")])]
    #[test_case("LMAO!", "LMAO!", vec![("pogu","__pog__u__url__")])]
    #[test_case("LMAO !pogu !clueless XDX!!!!!", "LMAO __pog__u__url__ __clueless_url__ XDX!!!!!", vec![("pogu","__pog__u__url__"), ("clueless", "__clueless_url__")])]
    #[tokio::test]
    async fn create_comment_alias(
        comment_text: &str,
        expected_comment_text: &str,
        aliases: Vec<(&'static str, &'static str)>,
    ) {
        let state = AppState::in_memory_db().await;
        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, String, String)> = conn
            .interact(move |conn| {
                for alias in aliases {
                    insert_alias(alias.0, alias.1, conn)?;
                }

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
            assert_eq!(text, expected_comment_text);
        });
    }
}
