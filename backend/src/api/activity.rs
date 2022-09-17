use anyhow::Context;
use axum::{routing::get, Router};
use axum::{Extension, Json};
use rusqlite::{params, Connection};
use serde::Serialize;
use serde_rusqlite::from_row;

use crate::api::error::Error;
use crate::api::{
    album::AlbumMetadata,
    comment::Comment,
    image::{
        get_all::{get_albums_containing_image, AllImagesImage},
        DbImage, Image,
    },
    user::{self, User},
};
use crate::AppState;

use super::album::{self, AlbumFilters};
use super::auth::Authorize;
use super::comment;
use std::sync::Arc;

use serde_with::skip_serializing_none;

pub fn api_route() -> Router {
    Router::new().route("/", get(get_activities))
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum Activity {
    Album(AlbumMetadata),
    Comment(Comment),
    User(User),
    Image(AllImagesImage),
}

pub fn get_new_images(conn: &Connection) -> Result<Vec<AllImagesImage>, Error> {
    let mut query = conn
        .prepare(
            "SELECT i.*, aia.created_at as published_at FROM images i \
            INNER JOIN album_image_associations aia ON i.key = aia.image_key \
            INNER JOIN albums a ON a.key = aia.album_key \
            WHERE a.published_at < aia.created_at",
        )
        .context("Failed to prepare statement for images query")?;

    let images = query
        .query_map(params![], |row| {
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

    Ok(all_images)
}

async fn get_activities(
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Activity>>, Error> {
    state
        .db
        .call(move |conn| {
            let filters = AlbumFilters {
                draft: false,
                ..Default::default()
            };
            let albums = album::get_all::get_albums(username, filters, conn)?
                .into_iter()
                .map(Activity::Album);

            let users = user::get_all(conn)?.into_iter().map(Activity::User);

            let comments = comment::get_all(conn)?.into_iter().map(Activity::Comment);

            let images = get_new_images(conn)?.into_iter().map(Activity::Image);

            let mut activities: Vec<Activity> =
                albums.chain(users).chain(comments).chain(images).collect();

            activities.sort_unstable_by(|a, b| b.cmp(a));

            Ok(Json(activities))
        })
        .await
}

// Activities should not contain images without publish date since those images are private.
impl PartialEq for Activity {
    fn eq(&self, other: &Activity) -> bool {
        use Activity::*;

        let this_time = match self {
            Album(a) => a.published_at,
            Comment(c) => c.created_at,
            User(u) => u.created_at,
            Image(i) => i.image.published_at.unwrap(),
        };

        let other_time = match other {
            Album(a) => a.published_at,
            Comment(c) => c.created_at,
            User(u) => u.created_at,
            Image(i) => i.image.published_at.unwrap(),
        };

        this_time == other_time
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Activity) -> Option<std::cmp::Ordering> {
        use Activity::*;

        let this_time = match self {
            Album(a) => a.published_at,
            Comment(c) => c.created_at,
            User(u) => u.created_at,
            Image(i) => i.image.published_at.unwrap(),
        };

        let other_time = match other {
            Album(a) => a.published_at,
            Comment(c) => c.created_at,
            User(u) => u.created_at,
            Image(i) => i.image.published_at.unwrap(),
        };

        this_time.partial_cmp(&other_time)
    }
}

impl Eq for Activity {}

impl Ord for Activity {
    fn cmp(&self, other: &Activity) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::api::image;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;
    use axum::extract::Path;

    #[tokio::test]
    async fn get_activity() {
        let state = AppState::in_memory_db().await;

        let (expected_user, expected_album, expected_image, expected_comment) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        author: &user,
                        published_at: 1,
                        ..Default::default()
                    },
                    conn,
                );

                let new_image_key = blob_uuid::random_blob();

                image::insert(
                    &DbImage {
                        key: new_image_key.clone(),
                        uploader: user.clone(),
                        uploaded_at: 2,

                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                // Insert an image that is not part of an album
                image::insert(
                    &DbImage {
                        key: blob_uuid::random_blob(),
                        uploader: user.clone(),
                        uploaded_at: 3,

                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                let comment = comment::insert_comment(
                    user.clone(),
                    String::from("foo"),
                    image,
                    album.clone(),
                    3,
                    conn,
                )
                .unwrap();

                (user, album, new_image_key, comment)
            })
            .await;

        let request = album::update::PutAlbumRequest {
            image_keys: Some(vec![expected_image.clone()]),
            ..Default::default()
        };
        album::update::put(
            Ok(Json(request)),
            Path(expected_album.clone()),
            Authorize(expected_user.clone()),
            Extension(state.clone()),
        )
        .await
        .unwrap();

        let result = get_activities(Authorize("".into()), Extension(state)).await;

        dbg!(&result);
        assert_matches!(result, Ok(Json(activities)) => {
            assert_matches!(&activities[0], Activity::Image(image) => {
                assert_eq!(expected_image, image.image.key);
                assert_eq!(expected_album, image.album_keys[0]);
            });

            assert_matches!(&activities[1], Activity::Comment(comment) => {
                assert_eq!(expected_comment.text, comment.text);
            });

            assert_matches!(&activities[2], Activity::Album(album) => {
                assert_eq!(expected_album, album.key);
            });

            assert_matches!(&activities[3], Activity::User(user) => {
                assert_eq!(expected_user, user.username);
            });

            // Ensure that non album images stay private
            for activity in activities {
                if let Activity::Image(image) = activity {
                    assert!(!image.album_keys.is_empty());
                }
            }

        });
    }
}
