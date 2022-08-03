use anyhow::Context;

use axum::{Extension, Json};

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::AppState;

use super::{DbImage, Image};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use serde_rusqlite::from_row;

use rusqlite::params;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AllImagesImage {
    pub album_keys: Vec<String>,

    #[serde(flatten)]
    pub image: Image,
}

pub(super) async fn get(
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<AllImagesImage>>, Error> {
    state
        .db
        .call(move |conn| {
            let mut query = conn
                .prepare("SELECT * FROM images WHERE uploader=?")
                .context("Failed to prepare statement for images query")?;

            let images = query
                .query_map(params![username], |row| {
                    Ok(Image::from_db(from_row::<DbImage>(row).unwrap()))
                })
                .context("Failed to query user images")?
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect user images")?;

            let mut all_images = Vec::new();
            for image in images {
                let album_keys = get_albums_containing_image(&image.key, conn)
                    .context("Failed to get albums for image")?;

                all_images.push(AllImagesImage { image, album_keys });
            }

            Ok(Json(all_images))
        })
        .await
}

fn get_albums_containing_image(
    key: &str,
    conn: &rusqlite::Connection,
) -> anyhow::Result<Vec<String>> {
    let mut query = conn
        .prepare("SELECT album_key FROM album_image_associations WHERE image_key=?")
        .context("Failed to prepare statement for image albums query")?;

    let albums = query
        .query_map(params![key], |row| Ok(from_row::<String>(row).unwrap()))
        .context("Failed to query image albums")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect image albums")?;

    Ok(albums)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn get_all_images_multiple_users() {
        let state = AppState::in_memory_db().await;

        let (bob, alice, expected_bob_images) = state
            .db
            .call(move |conn| {
                let bob = insert_user("bob", conn);
                let alice = insert_user("alice", conn);
                let images = vec![
                    insert_image(&bob, conn),
                    insert_image(&bob, conn),
                    insert_image(&bob, conn),
                ];

                (bob, alice, images)
            })
            .await;

        let result = get(Authorize(alice), Extension(state.clone())).await;
        assert_matches!(result, Ok(Json(images)) => {
            assert!(images.is_empty());
        });

        let result = get(Authorize(bob), Extension(state)).await;
        assert_matches!(result, Ok(Json(images)) => {
            let ids = images.into_iter().map(|i| i.image.key).collect::<Vec<_>>();

            assert_eq!(ids, expected_bob_images);
        });
    }

    #[tokio::test]
    async fn get_all_images_no_albums() {
        let state = AppState::in_memory_db().await;

        let (user, expected_images) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let images = vec![
                    insert_image(&user, conn),
                    insert_image(&user, conn),
                    insert_image(&user, conn),
                    insert_image(&user, conn),
                ];

                (user, images)
            })
            .await;

        let result = get(Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(images)) => {
            let ids = images.into_iter().map(|i| i.image.key).collect::<Vec<_>>();

            assert_eq!(ids, expected_images);
        });
    }

    #[tokio::test]
    async fn get_all_images_one_album() {
        let state = AppState::in_memory_db().await;

        let (user, expected_images, expected_album) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let images = vec![
                    insert_image(&user, conn),
                    insert_image(&user, conn),
                    insert_image(&user, conn),
                    insert_image(&user, conn),
                ];
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &images[0],
                        image_keys: &images,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                (user, images, album)
            })
            .await;

        let result = get(Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(Json(images)) => {
            let expected_albums = vec![expected_album];
            for image in &images {
                assert_eq!(&expected_albums, &image.album_keys);
            }

            let ids = images.into_iter().map(|i| i.image.key).collect::<Vec<_>>();
            assert_eq!(ids, expected_images);
        });
    }
}
