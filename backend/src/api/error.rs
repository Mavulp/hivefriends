use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    Placeholder,
    NotFound,
    InternalError(anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        eprintln!("{:#?}", self);
        let (status, error) = match self {
            Error::Placeholder => (StatusCode::NOT_IMPLEMENTED, "This is an error"),
            Error::NotFound => (StatusCode::NOT_FOUND, "This is an error"),
            Error::InternalError(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "This is an error"),
        };

        let body = Json(json!({
            "error": error,
        }))
        .into_response();

        (status, body).into_response()
    }
}
