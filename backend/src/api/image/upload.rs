use anyhow::Context;
use axum::{
    extract::{
        multipart::MultipartRejection, rejection::ContentLengthLimitRejection, ContentLengthLimit,
        Multipart,
    },
    Extension, Json,
};
use chrono::NaiveDateTime;
use image::DynamicImage;
use tokio::fs;
use tracing::warn;

use std::io::Cursor;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use super::orientation::ExifOrientation;
use super::{DbImage, DbImageMetadata, Image};
use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

const MB: u64 = 1024 * 1024;

pub(super) async fn post(
    multipart: Result<
        ContentLengthLimit<Multipart, { 25 * MB }>,
        ContentLengthLimitRejection<MultipartRejection>,
    >,
    Authorize(uploader): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Image>, Error> {
    let mut multipart = multipart?.0;

    let uploaded_at = SystemTime::UNIX_EPOCH
        .elapsed()
        .context("Failed to get timestamp")?
        .as_secs();

    let field = multipart
        .next_field()
        .await
        .context("Failed to read multipart field")?
        .ok_or(Error::NoImage)?;

    let file_name = field
        .file_name()
        .map(|s| s.to_owned())
        .unwrap_or_else(|| String::from("unknown"));
    let data = field.bytes().await.context("Failed to get image data")?;
    let size_bytes = data.len() as u64;

    let key = blob_uuid::random_blob();
    let image_key = key.clone();

    let mut metadata = DbImage {
        key,
        description: None,
        uploader,
        uploaded_at,
        metadata: DbImageMetadata {
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
        },
    };

    let mut orientation = None;

    let mut bufreader = std::io::BufReader::new(Cursor::new(&*data));
    let exifreader = exif::Reader::new();
    match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => {
            populate_metadata_from_exif(&mut metadata.metadata, &exif);
            orientation = exif
                .get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                .and_then(|f| {
                    if let exif::Value::Short(v) = &f.value {
                        ExifOrientation::try_from(v[0]).ok()
                    } else {
                        warn!("Unexpected format of camera model exif field");
                        None
                    }
                });
        }
        Err(e) => {
            warn!("Failed to read EXIF metadata: {}", e);
        }
    }

    store_image(
        state.data_path.clone(),
        &image_key,
        &metadata.metadata.file_name,
        &data,
        &orientation.unwrap_or(ExifOrientation::Normal),
    )
    .await?;

    let cmetadata = metadata.clone();
    let conn = state
        .pool
        .get()
        .await
        .context("Failed getting DB connection")?;
    conn.interact(move |conn| super::insert(&cmetadata, conn))
        .await
        .unwrap()?;

    Ok(Json(Image::from_db(metadata)))
}

fn populate_metadata_from_exif(metadata: &mut DbImageMetadata, exif: &exif::Exif) {
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
        .map(|f| f.display_value().with_unit(exif).to_string())
        .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok())
        .map(|t| t.timestamp() as i64);

    metadata.camera_brand = exif.get_field(Tag::Make, In::PRIMARY).and_then(|f| {
        if let exif::Value::Ascii(v) = &f.value {
            Some(String::from_utf8_lossy(&v[0]).to_string())
        } else {
            warn!("Unexpected format of camera brand exif field");
            None
        }
    });
    metadata.camera_model = exif.get_field(Tag::Model, In::PRIMARY).and_then(|f| {
        if let exif::Value::Ascii(v) = &f.value {
            Some(String::from_utf8_lossy(&v[0]).to_string())
        } else {
            warn!("Unexpected format of camera model exif field");
            None
        }
    });
    metadata.exposure_time = exif
        .get_field(Tag::ExposureTime, In::PRIMARY)
        .map(|f| f.display_value().with_unit(exif).to_string());
    metadata.f_number = exif
        .get_field(Tag::FNumber, In::PRIMARY)
        .map(|f| f.display_value().with_unit(exif).to_string());
    metadata.focal_length = exif
        .get_field(Tag::FocalLength, In::PRIMARY)
        .map(|f| f.display_value().with_unit(exif).to_string());
}

fn value_to_deg(value: &exif::Value) -> Option<f64> {
    if let exif::Value::Rational(parts) = value {
        Some(parts[0].to_f64() + parts[1].to_f64() / 60.0 + parts[2].to_f64() / 3600.0)
    } else {
        warn!("Unexpected format of latitude, ignoring");
        None
    }
}

enum ImageKind {
    Generated(Arc<DynamicImage>),
    Symlink(Arc<DynamicImage>),
}

impl ImageKind {
    fn image(&self) -> Arc<DynamicImage> {
        match self {
            ImageKind::Generated(i) => i.clone(),
            ImageKind::Symlink(i) => i.clone(),
        }
    }
}

fn generate_or_symlink_image(
    image: &ImageKind,
    source_width: u32,
    source_height: u32,
    target_width: u32,
    target_height: u32,
) -> ImageKind {
    if source_width < target_width && source_height < target_height {
        ImageKind::Symlink(image.image())
    } else {
        ImageKind::Generated(Arc::new(
            image.image().thumbnail(target_width, target_height),
        ))
    }
}

async fn store_image(
    directory: PathBuf,
    key: &str,
    file_name: &str,
    data: &[u8],
    orientation: &ExifOrientation,
) -> Result<(), Error> {
    let image = image::load_from_memory(data).map_err(Error::ImageError)?;
    let image = orientation.apply_to_image(image);
    let (width, height) = (image.width(), image.height());
    let image = ImageKind::Generated(Arc::new(image));

    let large = generate_or_symlink_image(&image, width, height, 1280, 1280);
    let medium = generate_or_symlink_image(&large, width, height, 800, 800);
    let tiny = generate_or_symlink_image(&medium, width, height, 360, 360);

    let mut image_dir = directory;
    image_dir.push(key);

    let mut original_path = image_dir.clone();
    original_path.push("original");

    fs::create_dir_all(&original_path)
        .await
        .context("Failed to create image directory")?;
    original_path.push(file_name);

    fs::write(original_path, &data)
        .await
        .context("Failed to write original file")?;

    let mut buffer = Vec::new();

    for (image, name) in &[
        (image, "full.jpg"),
        (large, "large.jpg"),
        (medium, "medium.jpg"),
        (tiny, "tiny.jpg"),
    ] {
        let mut path = image_dir.clone();
        path.push(name);

        match image {
            ImageKind::Generated(image) => {
                buffer.clear();
                let mut cursor = Cursor::new(&mut buffer);
                image.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(100))?;
                fs::write(path, &buffer)
                    .await
                    .context("Failed to write jpg")?;
            }
            ImageKind::Symlink(_) => {
                symlink("full.jpg", path).context("Failed to create symlink")?;
            }
        }
    }

    Ok(())
}
