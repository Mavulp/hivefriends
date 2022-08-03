use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Deserializer, Serialize};
use serde_rusqlite::{from_row, to_params_named};

mod delete_image;
mod get_all;
mod get_by_key;
mod orientation;
mod update_metadata;
mod upload;

use crate::api::error::Error;

const MAXIMUM_FILE_NAME_LENGTH: u64 = 96;
const MAXIMUM_DESCRIPTION_LENGTH: u64 = 256;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(upload::post))
        .route("/:key", get(get_by_key::get))
        .route("/:key", put(update_metadata::put))
        .route("/:key", delete(delete_image::delete))
        .route("/", get(get_all::get))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMetadata {
    file_name: String,
    size_bytes: u64,
    taken_at: Option<i64>,
    #[serde(default, deserialize_with = "non_empty_location")]
    location: Option<Location>,
    camera_brand: Option<String>,
    camera_model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    focal_length: Option<String>,
}

pub(super) fn non_empty_location<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Location>, D::Error> {
    let o: Option<Location> = Option::deserialize(d)?;
    Ok(o.filter(|l| !l.latitude.is_empty() && !l.longitude.is_empty()))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub key: String,
    pub description: Option<String>,
    pub uploader: String,
    pub uploaded_at: u64,
    pub published_at: Option<u64>,

    #[serde(flatten)]
    pub metadata: ImageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Location {
    latitude: String,
    longitude: String,
}

impl Image {
    pub fn from_db(db_metadata: DbImage) -> Self {
        Self {
            key: db_metadata.key,
            description: db_metadata.description,
            uploader: db_metadata.uploader,
            uploaded_at: db_metadata.uploaded_at,
            published_at: db_metadata.published_at,
            metadata: db_metadata.metadata.into(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DbImageMetadata {
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
}

impl From<DbImageMetadata> for ImageMetadata {
    fn from(meta: DbImageMetadata) -> Self {
        ImageMetadata {
            file_name: meta.file_name,
            size_bytes: meta.size_bytes,
            taken_at: meta.taken_at,
            location: if let (Some(latitude), Some(longitude)) =
                (meta.location_latitude, meta.location_longitude)
            {
                Some(Location {
                    latitude,
                    longitude,
                })
            } else {
                None
            },
            camera_brand: meta.camera_brand,
            camera_model: meta.camera_model,
            exposure_time: meta.exposure_time,
            f_number: meta.f_number,
            focal_length: meta.focal_length,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DbImage {
    pub key: String,
    pub description: Option<String>,
    pub uploader: String,
    pub uploaded_at: u64,
    #[serde(skip_serializing)]
    pub published_at: Option<u64>,

    #[serde(flatten)]
    pub metadata: DbImageMetadata,
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

pub fn insert(metadata: &DbImage, conn: &Connection) -> anyhow::Result<()> {
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

pub fn is_owner(image_key: &str, user: &str, conn: &Connection) -> Result<bool, Error> {
    let result = conn.query_row(
        "SELECT uploader FROM images WHERE key = ?1",
        params![image_key],
        |row| row.get::<_, String>(0),
    );

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        Err(Error::NotFound)
    } else {
        let uploader = result.map_err(anyhow::Error::new)?;

        Ok(uploader == user)
    }
}

pub fn select_image(key: &str, conn: &Connection) -> anyhow::Result<Option<DbImage>> {
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
            |row| Ok(from_row::<DbImage>(row).unwrap()),
        )
        .optional()?)
}

#[cfg(test)]
mod test {}
