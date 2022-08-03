use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use serde::Serialize;

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateShareTokenResponse {
    token: String,
}

pub(super) async fn post(
    Path(album_key): Path<String>,
    Authorize(username): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<CreateShareTokenResponse>, Error> {
    let now = SystemTime::UNIX_EPOCH
        .elapsed()
        .context("Failed to get current time")?
        .as_secs();
    let token = blob_uuid::random_blob();

    let share_token = token.clone();
    state
        .db
        .call::<_, Result<_, Error>>(move |conn| {
            let tx = conn.transaction().context("Failed to create transaction")?;

            super::insert_share_token(
                super::InsertShareToken {
                    share_token: &share_token,
                    album_key: &album_key,
                    created_by: &username,
                    created_at: now,
                },
                &tx,
            )?;

            tx.commit().context("Failed to commit transaction")?;

            Ok(())
        })
        .await?;

    Ok(Json(CreateShareTokenResponse { token }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn share_album() {
        let state = AppState::in_memory_db().await;

        let (user, album_key) = state
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

                (user, album_key)
            })
            .await;

        let result = post(Path(album_key), Authorize(user), Extension(state)).await;

        assert_matches!(result, Ok(_));
    }
}
