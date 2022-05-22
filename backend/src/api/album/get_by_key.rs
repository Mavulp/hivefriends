use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;

use serde_rusqlite::from_row;

use std::sync::Arc;

use crate::api::auth::Authorize;
use crate::api::error::Error;
use crate::api::image::{DbImageMetadata, ImageMetadata};
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::{DbAlbum, Timeframe};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AlbumResponse {
    key: String,
    title: String,
    description: Option<String>,
    cover_key: String,
    locations: Option<String>,
    author: String,
    draft: bool,
    timeframe: Timeframe,
    created_at: u64,
    images: Vec<ImageMetadata>,
    tagged_users: Vec<String>,
}

pub(super) async fn get<D: SqliteDatabase>(
    Path(album_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<AlbumResponse>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let result = conn
            .query_row(
                "SELECT \
                    key, \
                    title, \
                    description, \
                    cover_key, \
                    locations, \
                    author, \
                    draft, \
                    timeframe_from, \
                    timeframe_to, \
                    created_at \
                FROM albums \
                WHERE key=?1",
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
                    WHERE aia.album_key=?1",
                )
                .context("Failed to prepare statement for image query")?;
            let image_iter = stmt
                .query_map(params![db_album.key], |row| {
                    Ok(ImageMetadata::from_db(
                        from_row::<DbImageMetadata>(row).unwrap(),
                    ))
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
                key: album_key,
                title: db_album.title,
                description: db_album.description,
                cover_key: db_album.cover_key,
                locations: db_album.locations,
                author: db_album.author,
                draft: db_album.draft,
                timeframe: Timeframe {
                    from: db_album.timeframe_from,
                    to: db_album.timeframe_to,
                },
                created_at: db_album.created_at,
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
