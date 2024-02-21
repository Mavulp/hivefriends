use anyhow::Context;
use axum::{Extension, Json};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;

use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::error::Error;
use crate::api::image::{DbImage, Image};
use crate::api::public_auth::PublicAuthorize;
use crate::AppState;

use super::{DbAlbum, Timeframe};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AlbumResponse {
    key: String,
    title: String,
    description: Option<String>,
    cover_key: String,
    author: String,
    draft: bool,
    timeframe: Timeframe,
    published_at: u64,
    images: Vec<Image>,
    tagged_users: Vec<String>,
}

pub(super) async fn get(
    PublicAuthorize(album_key): PublicAuthorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<AlbumResponse>, Error> {
    state
        .db
        .call(move |conn| {
            let result = conn
                .query_row(
                    "SELECT \
                    a.key, \
                    a.title, \
                    a.description, \
                    a.cover_key, \
                    a.author, \
                    a.draft, \
                    a.timeframe_from, \
                    a.timeframe_to, \
                    a.published_at \
                FROM albums a \
                WHERE a.key=?1",
                    params![album_key],
                    |row| Ok(from_row::<DbAlbum>(row).unwrap()),
                )
                .optional()
                .context("Failed to query albums")?;

            if let Some(db_album) = result {
                let mut stmt = conn
                    .prepare(
                        "SELECT \
                        i.key, \
                        i.uploader, \
                        i.uploaded_at, \
                        i.file_name, \
                        i.size_bytes, \
                        i.taken_at, \
                        i.location_latitude, \
                        i.location_longitude, \
                        i.camera_brand, \
                        i.camera_model, \
                        i.exposure_time, \
                        i.f_number, \
                        i.focal_length \
                    FROM images i \
                    INNER JOIN album_image_associations aia ON aia.image_key=i.key \
                    WHERE aia.album_key=?1
                    ORDER BY aia.idx",
                    )
                    .context("Failed to prepare statement for image query")?;
                let image_iter = stmt
                    .query_map(params![db_album.key], |row| {
                        Ok(Image::from_db(from_row::<DbImage>(row).unwrap()))
                    })
                    .context("Failed to query images for album")?;

                let images = image_iter
                    .collect::<Result<Vec<_>, _>>()
                    .context("Failed to collect album images")?;

                let mut stmt = conn
                    .prepare(
                        "SELECT username FROM user_album_associations \
                    WHERE album_key = ?1",
                    )
                    .context("Failed to prepare statement for album query")?;
                let tagged_users = stmt
                    .query_map(params![&db_album.key], |row| row.get(0))
                    .context("Failed to query images")?
                    .collect::<Result<Vec<String>, _>>()
                    .context("Failed to collect tagged users")?;

                Ok(Json(AlbumResponse {
                    key: db_album.key,
                    title: db_album.title,
                    description: db_album.description,
                    cover_key: db_album.cover_key,
                    author: db_album.author,
                    draft: db_album.draft,
                    timeframe: Timeframe {
                        from: db_album.timeframe_from,
                        to: db_album.timeframe_to,
                    },
                    published_at: db_album.published_at,
                    images,
                    tagged_users,
                }))
            } else {
                Err(Error::NotFound)
            }
        })
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::{InsertAlbum, InsertShareToken};
    use crate::util::test::{insert_album, insert_image, insert_share_token, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn get_album_by_token() {
        let state = AppState::in_memory_db().await;

        let expected_album_key = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn);
                let image = insert_image(&user, conn);
                let album_key = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                );

                insert_share_token(
                    InsertShareToken {
                        album_key: &album_key,
                        created_by: &user,
                        ..Default::default()
                    },
                    conn,
                );

                album_key
            })
            .await;

        let result = get(
            PublicAuthorize(expected_album_key.clone()),
            Extension(state),
        )
        .await;

        assert_matches!(result, Ok(Json(album)) => {
            assert_eq!(album.key, expected_album_key);
        });
    }
}
