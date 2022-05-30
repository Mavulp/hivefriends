use axum::{
    async_trait,
    extract::{rejection::ExtensionRejection, FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use rusqlite::{params, OptionalExtension};
use serde::Deserialize;
use serde_json::json;
use serde_rusqlite::from_row;
use thiserror::Error;
use tracing::error;

use std::sync::Arc;

use crate::AppState;

pub struct PublicAuthorize(pub String);

#[derive(Debug, Deserialize)]
struct PublicPath {
    album: String,
    token: String,
}

use axum::extract::rejection::PathRejection;
use axum::extract::Path;

#[async_trait]
impl<B> FromRequest<B> for PublicAuthorize
where
    B: Send,
{
    type Rejection = PublicAuthorizationRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Path(path) = Path::<PublicPath>::from_request(req).await?;
        let Extension(state) = Extension::<Arc<AppState>>::from_request(req).await?;

        let conn = state.pool.get().await.map_err(anyhow::Error::new)?;

        let album_key = conn
            .interact(move |conn| {
                conn.query_row(
                    r"SELECT album_key FROM album_share_tokens WHERE share_token=?1",
                    params![path.token],
                    |row| Ok(from_row::<String>(row).unwrap()),
                )
                .optional()
            })
            .await
            .unwrap()
            .map_err(anyhow::Error::new)?;

        if let Some(album_key) = album_key {
            if album_key == path.album {
                Ok(PublicAuthorize(album_key))
            } else {
                Err(PublicAuthorizationRejection::WrongToken)
            }
        } else {
            Err(PublicAuthorizationRejection::InvalidToken)
        }
    }
}

#[derive(Debug, Error)]
pub enum PublicAuthorizationRejection {
    #[error("{0}")]
    Extension(#[from] ExtensionRejection),
    #[error("{0}")]
    Path(#[from] PathRejection),
    #[error("Share token is not valid for this album")]
    WrongToken,
    #[error("Invalid share token")]
    InvalidToken,
    #[error("{0}")]
    Generic(#[from] anyhow::Error),
}

impl IntoResponse for PublicAuthorizationRejection {
    fn into_response(self) -> Response {
        let status = match &self {
            PublicAuthorizationRejection::Path(_)
            | PublicAuthorizationRejection::Extension(_)
            | PublicAuthorizationRejection::Generic(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PublicAuthorizationRejection::WrongToken
            | PublicAuthorizationRejection::InvalidToken => StatusCode::UNAUTHORIZED,
        };

        let body = Json(json!({
            "message": self.to_string(),
        }))
        .into_response();

        (status, body).into_response()
    }
}
