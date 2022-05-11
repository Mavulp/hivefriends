use axum::{
    async_trait,
    extract::{
        rejection::{ExtensionRejection, TypedHeaderRejection},
        FromRequest, RequestParts,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json, TypedHeader,
};
use headers::{authorization::Bearer, Authorization};
use rusqlite::{params, OptionalExtension};
use serde_json::json;
use thiserror::Error;

use std::sync::Arc;

use crate::AppState;

pub struct Authorize(pub i64);

#[async_trait]
impl<B> FromRequest<B> for Authorize
where
    B: Send,
{
    type Rejection = AuthorizationRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req).await?;
        let Extension(state) = Extension::<Arc<AppState>>::from_request(req).await?;

        let conn = state.pool.get().await.map_err(anyhow::Error::new)?;

        let user_id = conn
            .interact(move |conn| {
                conn.query_row(
                    r"SELECT user_id FROM auth_sessions WHERE token=?1",
                    params![bearer.token()],
                    |row| row.get::<_, i64>(0),
                )
                .optional()
            })
            .await
            .unwrap()
            .map_err(anyhow::Error::new)?;

        if let Some(user_id) = user_id {
            Ok(Authorize(user_id))
        } else {
            Err(AuthorizationRejection::InvalidToken)
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationRejection {
    #[error("{0}")]
    Extension(#[from] ExtensionRejection),
    #[error("{0}")]
    Headers(#[from] TypedHeaderRejection),
    #[error("Bearer token is invalid")]
    InvalidToken,
    #[error("{0}")]
    Generic(#[from] anyhow::Error),
}

impl IntoResponse for AuthorizationRejection {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthorizationRejection::Extension(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthorizationRejection::Generic(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthorizationRejection::Headers(_) => StatusCode::BAD_REQUEST,
            AuthorizationRejection::InvalidToken => StatusCode::UNAUTHORIZED,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }))
        .into_response();

        (status, body).into_response()
    }
}
