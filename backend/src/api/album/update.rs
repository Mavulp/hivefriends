use anyhow::Context;
use axum::{extract::rejection::JsonRejection, extract::Path, Extension, Json};
use rusqlite::{params, ToSql};
use serde::Deserialize;
use serde_rusqlite::from_row;

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error, image::image_exists, user::user_exists};
use crate::util::{check_length, non_empty_str};
use crate::AppState;

use super::Timeframe;

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutAlbumRequest {
    #[serde(default, deserialize_with = "non_empty_str")]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub cover_key: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    pub author: Option<String>,
    pub draft: Option<bool>,
    pub timeframe: Option<Timeframe>,
    pub created_at: Option<u64>,
    pub image_keys: Option<Vec<String>>,
    pub tagged_users: Option<Vec<String>>,
}

pub async fn put(
    request: Result<Json<PutAlbumRequest>, JsonRejection>,
    Path(album_key): Path<String>,
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<&'static str>, Error> {
    let Json(mut request) = request?;

    check_length(
        "title",
        request.title.as_deref(),
        super::MAXIMUM_TITLE_LENGTH,
    )?;

    check_length(
        "description",
        request.description.as_deref(),
        super::MAXIMUM_DESCRIPTION_LENGTH,
    )?;

    state
        .db
        .call(move |conn| {
            let tx = conn.transaction().context("Failed to create transaction")?;
            if !super::is_owner(&album_key, &username, &tx)? {
                return Err(Error::Unathorized);
            }

            if let Some(cover_key) = &request.cover_key {
                if !image_exists(cover_key, &tx)? {
                    return Err(Error::InvalidKey);
                }
            }

            if let Some(want_draft) = request.draft {
                let is_draft = tx
                    .query_row(
                        "SELECT draft FROM albums WHERE key = ?",
                        params![album_key],
                        |row| Ok(from_row::<bool>(row).unwrap()),
                    )
                    .context("Failed to remove album image associations")?;

                if !is_draft {
                    if want_draft {
                        return Err(Error::AlreadyPublished);
                    } else {
                        // Ignore this request to not publish again
                        request.draft = None;
                    }
                }
            }

            if let Some(image_keys) = &request.image_keys {
                tx.execute(
                    "DELETE FROM album_image_associations WHERE album_key = ?",
                    params![album_key],
                )
                .context("Failed to remove album image associations")?;

                for (idx, image_key) in (0_i64..).zip(image_keys.iter()) {
                    if !image_exists(image_key, &tx)? {
                        return Err(Error::InvalidKey);
                    }

                    let now = SystemTime::UNIX_EPOCH
                        .elapsed()
                        .context("Failed to get current time")?
                        .as_secs();

                    tx.execute(
                        "INSERT INTO album_image_associations (
                            album_key, \
                             idx, \
                             image_key, \
                             created_at \
                        ) \
                        SELECT ?1, ?2, key, ?4 FROM images WHERE key = ?3",
                        params![album_key, idx, image_key, now],
                    )
                    .context("Failed to insert album image associations")?;
                }
            }

            if let Some(tagged_users) = &request.tagged_users {
                tx.execute(
                    "DELETE FROM user_album_associations WHERE album_key = ?",
                    params![album_key],
                )
                .context("Failed to remove album image associations")?;

                for tagged_user in tagged_users {
                    if !user_exists(tagged_user, &tx)? {
                        return Err(Error::InvalidUsername);
                    }

                    tx.execute(
                        "INSERT INTO user_album_associations (album_key, username) VALUES (?1, ?2)",
                        params![album_key, tagged_user],
                    )
                    .context("Failed to insert album user associations")?;
                }
            }

            let update_str = request.album_update_str();
            if !update_str.is_empty() {
                let mut params = request.update_params()?;
                params.push(Box::new(album_key));
                if let Err(rusqlite::Error::SqliteFailure(e, _)) = tx.query_row(
                    &format!("UPDATE albums SET {update_str} WHERE key = ?"),
                    rusqlite::params_from_iter(params.iter()),
                    |_| Ok(()),
                ) {
                    match e.code {
                        rusqlite::ErrorCode::ConstraintViolation => Err(Error::InvalidKey),
                        _ => panic!(),
                    }
                } else {
                    tx.commit().context("Failed to commit transaction")?;
                    Ok(())
                }
            } else {
                tx.commit().context("Failed to commit transaction")?;
                Ok(())
            }
        })
        .await?;

    Ok(Json("Success"))
}

impl PutAlbumRequest {
    fn album_update_str(&self) -> String {
        let mut result = Vec::new();

        if self.title.is_some() {
            result.push("title = ?");
        }

        if self.description.is_some() {
            result.push("description = ?");
        }

        if self.cover_key.is_some() {
            result.push("cover_key = ?");
        }

        if self.draft.is_some() {
            result.push("draft = ?");
            result.push("published_at = ?");
        }

        if self.timeframe.is_some() {
            result.push("timeframe_from = ?");
            result.push("timeframe_to = ?");
        }

        result.join(", ")
    }

    fn update_params(mut self) -> Result<Vec<Box<dyn ToSql>>, Error> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(title) = self.title.take() {
            params.push(Box::new(title))
        }

        if let Some(description) = self.description.take() {
            params.push(Box::new(description));
        }

        if let Some(cover_key) = self.cover_key.take() {
            params.push(Box::new(cover_key));
        }

        if let Some(draft) = self.draft.take() {
            params.push(Box::new(draft));

            let now = SystemTime::UNIX_EPOCH
                .elapsed()
                .context("Failed to get current time")?
                .as_secs();
            params.push(Box::new(now));
        }

        if let Some(timeframe) = self.timeframe.take() {
            params.push(Box::new(timeframe.from));
            params.push(Box::new(timeframe.to));
        }

        Ok(params)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::{get_album, InsertAlbum};
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn published_to_draft() {
        let state = AppState::in_memory_db().await;

        let (key, user) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);

                let album = insert_album(
                    InsertAlbum {
                        draft: false,
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                (album, user)
            })
            .await;

        let request = PutAlbumRequest {
            draft: Some(true),
            ..Default::default()
        };

        let result = put(
            Ok(Json(request)),
            Path(key),
            Authorize(user),
            Extension(state),
        )
        .await;

        assert_matches!(result, Err(Error::AlreadyPublished));
    }

    #[tokio::test]
    async fn published_to_published() {
        let state = AppState::in_memory_db().await;

        let (key, user) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);

                let album = insert_album(
                    InsertAlbum {
                        draft: false,
                        published_at: 42,
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                (album, user)
            })
            .await;

        let request = PutAlbumRequest {
            draft: Some(false),
            ..Default::default()
        };

        put(
            Ok(Json(request)),
            Path(key.clone()),
            Authorize(user),
            Extension(state.clone()),
        )
        .await
        .unwrap();

        let album = state
            .db
            .call(move |conn| get_album(&key, conn))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(album.published_at, 42);
    }
}
