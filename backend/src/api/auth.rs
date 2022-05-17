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
use serde::Deserialize;
use serde_json::json;
use serde_rusqlite::from_row;
use thiserror::Error;
use tracing::error;

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::AppState;

pub struct Authorize(pub String);

#[derive(Deserialize)]
struct DbSession {
    user_key: String,
    created_at: u64,
}

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

        let bearer_token = bearer.token().to_owned();
        let db_session = conn
            .interact(move |conn| {
                conn.query_row(
                    r"SELECT user_key, created_at FROM auth_sessions WHERE token=?1",
                    params![bearer.token()],
                    |row| Ok(from_row::<DbSession>(row).unwrap()),
                )
                .optional()
            })
            .await
            .unwrap()
            .map_err(anyhow::Error::new)?;

        if let Some(session) = db_session {
            let created_at = Duration::from_secs(session.created_at);
            let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();

            if now < created_at + Duration::from_secs(crate::AUTH_TIME_SECONDS) {
                Ok(Authorize(session.user_key))
            } else {
                conn.interact(move |conn| {
                    if let Err(e) = conn.execute(
                        "DELETE FROM auth_sessions WHERE token=?1",
                        params![bearer_token],
                    ) {
                        error!("Failed to delete auth token: {}", e);
                    }
                })
                .await
                .unwrap();

                Err(AuthorizationRejection::ExpiredToken)
            }
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
    #[error("Bearer token has expired, login again")]
    ExpiredToken,
    #[error("{0}")]
    Generic(#[from] anyhow::Error),
}

impl IntoResponse for AuthorizationRejection {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthorizationRejection::Extension(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthorizationRejection::Generic(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthorizationRejection::Headers(_) => StatusCode::BAD_REQUEST,
            AuthorizationRejection::InvalidToken | AuthorizationRejection::ExpiredToken => {
                StatusCode::UNAUTHORIZED
            }
        };

        let body = Json(json!({
            "error": self.to_string(),
        }))
        .into_response();

        (status, body).into_response()
    }
}
