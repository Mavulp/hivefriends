use axum::{
    routing::{get, post},
    Router,
};

mod get_by_id;
mod upload;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(upload::post))
        .route("/:key", get(get_by_id::get))
}
