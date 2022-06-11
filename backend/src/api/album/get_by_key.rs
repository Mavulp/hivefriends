use anyhow::Context;
use axum::{extract::Path, Extension, Json};

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Album;

pub(super) async fn get<D: SqliteDatabase>(
    Path(album_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Album>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| match super::get_album(&album_key, conn)? {
        Some(album) => Ok(Json(album)),
        None => Err(Error::NotFound),
    })
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn get_album_tagged_users() {
        let state = AppState::in_memory_db().await;

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<(String, Vec<String>)> = conn
            .interact(move |conn| {
                let users = vec![insert_user("test", conn)?, insert_user("test2", conn)?];
                let user = users[0].clone();
                let image = insert_image(&user, conn).context("")?;
                let album = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )?;

                Ok((album, users))
            })
            .await;

        let (album, users) = result.unwrap();

        let result = get(Path(album), Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(album)) => {
            assert_eq!(album.tagged_users, users);
        });
    }
}
