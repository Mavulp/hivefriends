use crate::FileDb;

use axum::{
    routing::{get, post, put},
    Router,
};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_row, to_params_named};

mod get_by_key;
mod orientation;
mod update_metadata;
mod upload;

const MAXIMUM_FILE_NAME_LENGTH: u64 = 96;
const MAXIMUM_DESCRIPTION_LENGTH: u64 = 600;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(upload::post))
        .route("/:key", get(get_by_key::get))
        .route("/:key", put(update_metadata::put::<FileDb>))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ImageMetadata {
    key: String,

    file_name: String,
    size_bytes: u64,
    taken_at: Option<i64>,
    location: Option<Location>,
    camera_brand: Option<String>,
    camera_model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    focal_length: Option<String>,

    description: Option<String>,
    uploader: String,
    uploaded_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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

            description: db_metadata.description,
            uploader: db_metadata.uploader,
            uploaded_at: db_metadata.uploaded_at,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DbImageMetadata {
    pub key: String,

    pub file_name: String,
    pub size_bytes: u64,
    pub taken_at: Option<i64>,
    pub location_latitude: Option<String>,
    pub location_longitude: Option<String>,
    pub camera_brand: Option<String>,
    pub camera_model: Option<String>,
    pub exposure_time: Option<String>,
    pub f_number: Option<String>,
    pub focal_length: Option<String>,

    pub description: Option<String>,
    pub uploader: String,
    pub uploaded_at: u64,
}

pub fn image_exists(key: &str, conn: &Connection) -> anyhow::Result<bool> {
    let result = conn.query_row("SELECT 1 FROM images WHERE key = ?1", params![key], |_| {
        Ok(())
    });

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        Ok(false)
    } else {
        result?;

        Ok(true)
    }
}

pub fn comment_exists(id: i64, conn: &Connection) -> anyhow::Result<bool> {
    let result = conn.query_row("SELECT 1 FROM comments WHERE id = ?1", params![id], |_| {
        Ok(())
    });

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        Ok(false)
    } else {
        result?;

        Ok(true)
    }
}

pub fn insert(metadata: &DbImageMetadata, conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO images ( \
            key, \
            uploader, \
            file_name, \
            size_bytes, \
            taken_at, \
            location_latitude, \
            location_longitude, \
            camera_brand, \
            camera_model, \
            exposure_time, \
            f_number, \
            focal_length, \
            description, \
            uploaded_at \
        ) VALUES ( \
            :key, \
            :uploader, \
            :file_name, \
            :size_bytes, \
            :taken_at, \
            :location_latitude, \
            :location_longitude, \
            :camera_brand, \
            :camera_model, \
            :exposure_time, \
            :f_number, \
            :focal_length, \
            :description, \
            :uploaded_at \
        )",
        to_params_named(&metadata).unwrap().to_slice().as_slice(),
    )?;

    Ok(())
}

pub fn select_image(key: &str, conn: &Connection) -> anyhow::Result<Option<DbImageMetadata>> {
    Ok(conn
        .query_row(
            "SELECT \
            key, \
            uploader, \
            uploaded_at, \
            file_name, \
            size_bytes, \
            taken_at, \
            location_latitude, \
            location_longitude, \
            camera_brand, \
            camera_model, \
            exposure_time, \
            f_number, \
            focal_length, \
            description \
        FROM images \
        WHERE key = ?1",
            params![key],
            |row| Ok(from_row::<DbImageMetadata>(row).unwrap()),
        )
        .optional()?)
}

#[cfg(test)]
mod test {}
