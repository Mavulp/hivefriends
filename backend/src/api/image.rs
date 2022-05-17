use axum::{
    routing::{get, post},
    Router,
};

mod get_by_key;
mod upload;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(upload::post))
        .route("/:key", get(get_by_key::get))
}
