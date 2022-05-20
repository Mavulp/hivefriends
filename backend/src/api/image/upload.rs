use axum::{
    extract::{
        multipart::MultipartRejection, rejection::ContentLengthLimitRejection, ContentLengthLimit,
        Multipart,
    },
    Extension, Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_rusqlite::to_params_named;
use tokio::fs;
use tracing::warn;

use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use super::DbImageMetadata;
use crate::{api::auth::Authorize, api::error::Error, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ImageCreationResponse {
    key: String,
}

const MB: u64 = 1024 * 1024;

pub(super) async fn post(
    multipart: Result<
        ContentLengthLimit<Multipart, { 10 * MB }>,
        ContentLengthLimitRejection<MultipartRejection>,
    >,
    Authorize(user_key): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ImageCreationResponse>, Error> {
    let multipart = multipart?.0;

    match upload_image(multipart, user_key, &state).await {
        Ok(response) => Ok(Json(response)),
        Err(e) if e.is::<ImageCreationError>() => {
            match e.downcast_ref::<ImageCreationError>().unwrap() {
                ImageCreationError::NoImage => Err(Error::InvalidArguments(e)),
                ImageCreationError::ImageError(_) => Err(Error::InvalidArguments(e)),
            }
        }
        Err(e) => Err(Error::InternalError(e)),
    }
}

#[derive(Debug, thiserror::Error)]
enum ImageCreationError {
    #[error("Missing image data in multipart message")]
    NoImage,

    #[error("Failed to process image: {0}")]
    ImageError(#[from] image::ImageError),
}

async fn upload_image(
    mut multipart: Multipart,
    uploader_key: String,
    state: &Arc<AppState>,
) -> anyhow::Result<ImageCreationResponse> {
    let uploaded_at = SystemTime::UNIX_EPOCH.elapsed()?.as_secs();

    let field = multipart
        .next_field()
        .await?
        .ok_or(ImageCreationError::NoImage)?;

    let file_name = field.file_name().map(|s| s.to_owned());
    let data = field.bytes().await?;
    let size_bytes = data.len() as u64;

    let key = blob_uuid::random_blob();
    store_image(state.data_path.clone(), &key, &data).await?;

    let image_key = key.clone();

    let mut metadata = DbImageMetadata {
        key: image_key,
        uploader_key,
        file_name,
        size_bytes,
        taken_at: None,
        location_latitude: None,
        location_longitude: None,
        camera_brand: None,
        camera_model: None,
        exposure_time: None,
        f_number: None,
        focal_length: None,

        uploaded_at,
    };

    let mut bufreader = std::io::BufReader::new(Cursor::new(&*data));
    let exifreader = exif::Reader::new();
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        populate_metadata_from_exif(&mut metadata, exif);
    }

    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        conn.execute(
            "INSERT INTO images ( \
                key, \
                uploader_key, \
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
                uploaded_at \
            ) VALUES ( \
                :key, \
                :uploader_key, \
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
                :uploaded_at \
            )",
            to_params_named(&metadata).unwrap().to_slice().as_slice(),
        )
    })
    .await
    .unwrap()?;

    Ok(ImageCreationResponse { key })
}

fn populate_metadata_from_exif(metadata: &mut DbImageMetadata, exif: exif::Exif) {
    use exif::{In, Tag};

    let latitude_field = exif.get_field(Tag::GPSLatitude, In::PRIMARY);
    let longitude_field = exif.get_field(Tag::GPSLongitude, In::PRIMARY);

    let lat_ref = exif
        .get_field(Tag::GPSLatitudeRef, In::PRIMARY)
        .map(|f| f.display_value().to_string());

    let long_ref = exif
        .get_field(Tag::GPSLongitudeRef, In::PRIMARY)
        .map(|f| f.display_value().to_string());

    if let (Some(latitude), Some(longitude), Some(lat_ref), Some(long_ref)) =
        (latitude_field, longitude_field, lat_ref, long_ref)
    {
        let mut lat_deg = value_to_deg(&latitude.value);
        let mut long_deg = value_to_deg(&longitude.value);

        if lat_ref == "S" {
            lat_deg = lat_deg.map(|d| d * -1.0);
        }
        if long_ref == "W" {
            long_deg = long_deg.map(|d| d * -1.0);
        }

        if lat_deg.is_some() && long_deg.is_some() {
            metadata.location_latitude = lat_deg.map(|d| d.to_string());
            metadata.location_longitude = long_deg.map(|d| d.to_string());
        }
    };

    metadata.taken_at = exif
        .get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string())
        .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok())
        .map(|t| t.timestamp() as u64);

    metadata.camera_brand = exif
        .get_field(Tag::Make, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string())
        .map(|s| s.trim_matches('"').to_owned());
    metadata.camera_model = exif
        .get_field(Tag::Model, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string())
        .map(|s| s.trim_matches('"').to_owned());
    metadata.exposure_time = exif
        .get_field(Tag::ExposureTime, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string());
    metadata.f_number = exif
        .get_field(Tag::FNumber, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string());
    metadata.focal_length = exif
        .get_field(Tag::FocalLength, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string());
}

fn value_to_deg(value: &exif::Value) -> Option<f64> {
    if let exif::Value::Rational(parts) = value {
        Some(parts[0].to_f64() + parts[1].to_f64() / 60.0 + parts[2].to_f64() / 3600.0)
    } else {
        warn!("Unexpected format of latitude, ignoring");
        None
    }
}

async fn store_image(directory: PathBuf, key: &str, data: &[u8]) -> anyhow::Result<()> {
    let image = image::load_from_memory(data)?;
    let large = image.thumbnail(1280, 1280);
    let medium = large.thumbnail(800, 800);
    let tiny = medium.thumbnail(360, 360);

    let mut image_dir = directory;
    image_dir.push(key);

    fs::create_dir_all(&image_dir).await?;

    let mut buffer = Vec::new();

    for (image, name) in &[
        (image, "original.png"),
        (large, "large.png"),
        (medium, "medium.png"),
        (tiny, "tiny.png"),
    ] {
        let mut path = image_dir.clone();
        path.push(name);

        buffer.clear();
        let mut cursor = Cursor::new(&mut buffer);
        image.write_to(&mut cursor, image::ImageOutputFormat::Png)?;
        fs::write(path, &buffer).await?;
    }

    Ok(())
}
