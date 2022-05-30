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

    #[error("Unathorized")]
    Unathorized,

    #[error("Comment does not belong to specified image")]
    WrongImage,

    #[error("Invalid username")]
    InvalidUsername,

    #[error("Invalid username or password")]
    InvalidLogin,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("timeframe contains an invalid range")]
    InvalidTimeframe,

    #[error("coverKey is not a valid image key")]
    InvalidCoverKey,

    #[error("One of the album or image keys is not valid")]
    InvalidKey,

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
            Error::InvalidLogin | Error::InvalidPassword | Error::Unathorized => {
                StatusCode::UNAUTHORIZED
            }
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::InternalError(e) => {
                let err = e
                    .chain()
                    .skip(1)
                    .fold(e.to_string(), |acc, cause| format!("{}: {}\n", acc, cause));
                error!("API encountered error: {}", err);

                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::InvalidCoverKey
            | Error::InvalidKey
            | Error::InvalidTimeframe
            | Error::InvalidUsername
            | Error::WrongImage
            | Error::JsonRejection(_)
            | Error::MultipartSizeRejection(_)
            | Error::InvalidArguments(_) => StatusCode::BAD_REQUEST,
        };

        let message = if let Error::JsonRejection(rej) = self {
            use std::error::Error;
            match rej {
                JsonRejection::JsonDataError(e) => e.source().unwrap().to_string(),
                JsonRejection::JsonSyntaxError(e) => e.source().unwrap().to_string(),
                _ => rej.to_string(),
            }
        } else {
            self.to_string()
        };

        let body = Json(json!({
            "message": message,
        }));
        (status, body).into_response()
    }
}
