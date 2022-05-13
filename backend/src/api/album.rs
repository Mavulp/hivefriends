use axum::{
    routing::{get, post},
    Router,
};

use serde::{Deserialize, Serialize};

mod create;
mod get_all;
mod get_by_id;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(create::post))
        .route("/", get(get_all::get))
        .route("/:key", get(get_by_id::get))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Timeframe {
    from: Option<u64>,
    to: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    key: String,
    uploader_key: String,
    created_at: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Album {
    key: String,
    title: String,
    description: Option<String>,
    locations: Option<String>,
    timeframe: Timeframe,
    created_at: u64,
    images: Vec<Image>,
}

#[derive(Deserialize)]
struct DbAlbum {
    id: i64,
    key: String,
    title: String,
    description: Option<String>,
    locations: Option<String>,
    timeframe_from: Option<u64>,
    timeframe_to: Option<u64>,
    created_at: u64,
}
