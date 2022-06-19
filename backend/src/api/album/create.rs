use anyhow::Context;
use axum::{extract::rejection::JsonRejection, Extension, Json};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error, image::image_exists};
use crate::util::{check_length, non_empty_str};
use crate::AppState;

use super::Timeframe;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateAlbumRequest {
    title: String,
    #[serde(default, deserialize_with = "non_empty_str")]
    description: Option<String>,
    cover_key: String,
    timeframe: Timeframe,
    image_keys: Vec<String>,
    #[serde(default)]
    tagged_users: Vec<String>,
    #[serde(default)]
    draft: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateAlbumResponse {
    key: String,
}

pub(super) async fn post(
    request: Result<Json<CreateAlbumRequest>, JsonRejection>,
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<CreateAlbumResponse>, Error> {
    let Json(request) = request?;

    if let (Some(from), Some(to)) = (request.timeframe.from, request.timeframe.to) {
        if from > to {
            return Err(Error::InvalidTimeframe);
        }
    }

    check_length("title", Some(&request.title), super::MAXIMUM_TITLE_LENGTH)?;

    check_length(
        "description",
        request.description.as_deref(),
        super::MAXIMUM_DESCRIPTION_LENGTH,
    )?;

    let now = SystemTime::UNIX_EPOCH
        .elapsed()
        .context("Failed to get current time")?
        .as_secs();
    let key = blob_uuid::random_blob();

    let album_key = key.clone();
    state
        .db
        .call(move |conn| {
            if !image_exists(&request.cover_key, conn)? {
                return Err(Error::InvalidCoverKey);
            }

            let tx = conn.transaction().context("Failed to create transaction")?;

            super::insert_album(
                super::InsertAlbum {
                    key: &album_key,
                    title: &request.title,
                    description: request.description.as_deref(),
                    cover_key: &request.cover_key,
                    author: &username,
                    draft: request.draft,
                    timeframe_from: request.timeframe.from,
                    timeframe_to: request.timeframe.to,
                    created_at: now,
                    image_keys: &request.image_keys,
                    tagged_users: &request.tagged_users,
                },
                &tx,
            )?;

            tx.commit().context("Failed to commit transaction")?;

            Ok(())
        })
        .await?;

    Ok(Json(CreateAlbumResponse { key }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::test::{insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn create_album() {
        let state = AppState::in_memory_db().await;

        let (user_a, user_b, images) = state
            .db
            .call(move |conn| {
                let user_a = insert_user("test", conn).unwrap();
                let user_b = insert_user("test2", conn).unwrap();
                let images = vec![
                    insert_image(&user_a, conn).unwrap(),
                    insert_image(&user_a, conn).unwrap(),
                    insert_image(&user_a, conn).unwrap(),
                ];

                (user_a, user_b, images)
            })
            .await;

        let request = CreateAlbumRequest {
            title: "album".into(),
            description: Some("Description".into()),
            cover_key: images[0].clone(),
            timeframe: Timeframe {
                from: Some(0),
                to: Some(10),
            },
            image_keys: images,
            tagged_users: vec![user_a.clone(), user_b],
            draft: true,
        };

        let result = post(Ok(Json(request)), Authorize(user_a), Extension(state)).await;

        assert_matches!(result, Ok(_));
    }

    #[tokio::test]
    async fn create_album_cover_key_not_in_image_keys() {
        let state = AppState::in_memory_db().await;

        let user = state
            .db
            .call(move |conn| insert_user("test", conn).unwrap())
            .await;

        let request = CreateAlbumRequest {
            title: "album".into(),
            cover_key: "invalid key".into(),
            ..Default::default()
        };

        let result = post(Ok(Json(request)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::InvalidCoverKey));
    }

    #[tokio::test]
    async fn create_album_invalid_tagged_user() {
        let state = AppState::in_memory_db().await;

        let (user, image) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn).unwrap();
                let image = insert_image(&user, conn).unwrap();

                (user, image)
            })
            .await;

        let request = CreateAlbumRequest {
            title: "album".into(),
            cover_key: image.clone(),
            image_keys: vec![image],
            tagged_users: vec!["invalid-user".into()],
            ..Default::default()
        };

        let result = post(Ok(Json(request)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::InvalidUsername));
    }

    #[tokio::test]
    async fn create_album_invalid_timeframe() {
        let state = AppState::in_memory_db().await;

        let (user, image) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn).unwrap();
                let image = insert_image(&user, conn).unwrap();

                (user, image)
            })
            .await;

        let request = CreateAlbumRequest {
            title: "album".into(),
            cover_key: image.clone(),
            image_keys: vec![image],
            timeframe: Timeframe {
                from: Some(10),
                to: Some(0),
            },
            ..Default::default()
        };

        let result = post(Ok(Json(request)), Authorize(user), Extension(state)).await;

        assert_matches!(result, Err(Error::InvalidTimeframe));
    }
}
