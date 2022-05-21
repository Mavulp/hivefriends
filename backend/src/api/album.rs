use axum::{
    routing::{get, post},
    Router,
};

use serde::{Deserialize, Serialize};

mod create;
mod get_all;
mod get_by_key;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(create::post))
        .route("/", get(get_all::get))
        .route("/:key", get(get_by_key::get))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Timeframe {
    from: Option<u64>,
    to: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct DbAlbum {
    key: String,
    title: String,
    description: Option<String>,
    cover_key: String,
    locations: Option<String>,
    author: String,
    draft: bool,
    timeframe_from: Option<u64>,
    timeframe_to: Option<u64>,
    created_at: u64,
}
