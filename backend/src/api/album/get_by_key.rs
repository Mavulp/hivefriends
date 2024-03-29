use axum::{extract::Path, Extension, Json};

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::AppState;

use super::Album;

pub(super) async fn get(
    Path(album_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Album>, Error> {
    state
        .db
        .call(move |conn| match super::get_album(&album_key, conn)? {
            Some(album) => Ok(Json(album)),
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
    use test_case::test_case;

    #[test_case(&[0])]
    #[test_case(&[1])]
    #[test_case(&[5])]
    #[test_case(&[1, 0, 1])]
    #[test_case(&[1, 2, 3])]
    #[tokio::test]
    async fn get_album_comment_count(comment_count: &'static [u32]) {
        let state = AppState::in_memory_db().await;

        let (album, images) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);

                let mut images = Vec::new();
                for _ in comment_count {
                    images.push(insert_image(&user, conn));
                }

                let album = insert_album(
                    InsertAlbum {
                        cover_key: &images[0],
                        image_keys: &images,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                for (count, img) in comment_count.iter().zip(images.iter()) {
                    for _ in 0..*count {
                        insert_comment(&user, img, &album, "text", conn);
                    }
                }

                (album, images)
            })
            .await;

        let expected = images.iter().zip(comment_count).collect::<Vec<_>>();
        let result = get(Path(album), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(album)) => {
            let result = album.images.iter().map(|i| (&i.image.key, &i.comment_count)).collect::<Vec<_>>();
            assert_eq!(expected, result)
        });
    }

    #[tokio::test]
    async fn get_album_tagged_users() {
        let state = AppState::in_memory_db().await;

        let (album, users) = state
            .db
            .call(move |conn| {
                let users = vec![insert_user("test", conn), insert_user("test2", conn)];
                let user = users[0].clone();
                let image = insert_image(&user, conn);
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                (album, users)
            })
            .await;

        let result = get(Path(album), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(album)) => {
            assert_eq!(album.tagged_users, users);
        });
    }
}
