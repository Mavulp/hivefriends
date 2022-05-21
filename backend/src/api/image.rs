use crate::FileDb;
use anyhow::Context;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_row, to_params_named};

mod create_comment;
mod delete_comment;
mod get_all_comments;
mod get_by_key;
mod update_metadata;
mod upload;

pub fn api_route() -> Router {
    Router::new()
        .route("/", post(upload::post))
        .route("/:key", get(get_by_key::get))
        .route("/:key", put(update_metadata::put::<FileDb>))
        .route("/:key/comments", get(get_all_comments::get::<FileDb>))
        .route("/:key/comments", post(create_comment::post::<FileDb>))
        .route(
            "/:key/comments/:key",
            delete(delete_comment::delete::<FileDb>),
        )
}

#[derive(Eq, PartialEq, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: i64,
    author: String,
    image_key: String,
    created_at: u64,
    text: String,
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

    pub file_name: Option<String>,
    pub size_bytes: u64,
    pub taken_at: Option<u64>,
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

pub fn get_comment(id: i64, conn: &Connection) -> anyhow::Result<Option<Comment>> {
    let result = conn.query_row(
        "SELECT author, image_key, created_at, text FROM comments WHERE id = ?1",
        params![id],
        |row| {
            Ok(Comment {
                id,
                author: row.get(0)?,
                image_key: row.get(1)?,
                created_at: row.get(2)?,
                text: row.get(3)?,
            })
        },
    );

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        return Ok(None);
    }

    let comment = result.context("Failed to get comment")?;

    Ok(Some(comment))
}

pub fn insert_comment(
    author: String,
    text: String,
    image_key: String,
    created_at: u64,
    conn: &Connection,
) -> anyhow::Result<Comment> {
    conn.execute(
        "INSERT INTO comments (author, image_key, created_at, text) VALUES (?1, ?2, ?3, ?4)",
        params![&author, &image_key, created_at, text],
    )
    .context("Failed to insert comment")?;

    let last_id = conn.last_insert_rowid();

    Ok(Comment {
        id: last_id,
        text,
        author,
        image_key,
        created_at,
    })
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
