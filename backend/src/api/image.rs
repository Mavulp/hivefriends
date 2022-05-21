use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

mod get_by_key;
mod upload;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(upload::post))
        .route("/:key", get(get_by_key::get))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ImageMetadata {
    key: String,

    file_name: Option<String>,
    size_bytes: u64,
    taken_at: Option<u64>,
    location: Option<Location>,
    camera_brand: Option<String>,
    camera_model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    focal_length: Option<String>,

    uploader: String,
    uploaded_at: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Location {
    latitude: String,
    longitude: String,
}

impl ImageMetadata {
    pub fn from_db(db_metadata: DbImageMetadata) -> Self {
        let location = match (
            db_metadata.location_latitude,
            db_metadata.location_longitude,
        ) {
            (Some(latitude), Some(longitude)) => Some(Location {
                latitude,
                longitude,
            }),
            _ => None,
        };

        Self {
            key: db_metadata.key,

            file_name: db_metadata.file_name,
            size_bytes: db_metadata.size_bytes,
            taken_at: db_metadata.taken_at,
            location,
            camera_brand: db_metadata.camera_brand,
            camera_model: db_metadata.camera_model,
            exposure_time: db_metadata.exposure_time,
            f_number: db_metadata.f_number,
            focal_length: db_metadata.focal_length,

            uploader: db_metadata.uploader,
            uploaded_at: db_metadata.uploaded_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct DbImageMetadata {
    key: String,

    file_name: Option<String>,
    size_bytes: u64,
    taken_at: Option<u64>,
    location_latitude: Option<String>,
    location_longitude: Option<String>,
    camera_brand: Option<String>,
    camera_model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    focal_length: Option<String>,

    uploader: String,
    uploaded_at: u64,
}
