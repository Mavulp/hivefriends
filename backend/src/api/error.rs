use axum::{
    extract::{
        multipart::MultipartRejection,
        rejection::{ContentLengthLimitRejection, JsonRejection},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not Found")]
    NotFound,

    #[error("Invalid username or password")]
    InvalidLogin,

    #[error("Invalid argument(s): {0}")]
    InvalidArguments(anyhow::Error),

    #[error("Internal Server Error")]
    InternalError(#[from] anyhow::Error),

    #[error("{0}")]
    JsonRejection(#[from] JsonRejection),

    #[error("{0}")]
    MultipartSizeRejection(#[from] ContentLengthLimitRejection<MultipartRejection>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::InvalidLogin => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::InternalError(e) => {
                error!("API encountered error: {}", e);

                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::JsonRejection(_) => StatusCode::BAD_REQUEST,
            Error::MultipartSizeRejection(_) => StatusCode::BAD_REQUEST,
            Error::InvalidArguments(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }))
        .into_response();

        (status, body).into_response()
    }
}
