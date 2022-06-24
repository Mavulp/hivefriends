use anyhow::Context;
use axum::{
    extract::{rejection::JsonRejection, Path},
    Extension, Json,
};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};
use tokio::fs;

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::util::{check_length, non_empty_str};
use crate::AppState;

use super::Location;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PutImageMetadataRequest {
    #[serde(default, deserialize_with = "non_empty_str")]
    file_name: Option<String>,
    taken_at: Option<i64>,
    location: Option<Location>,
    #[serde(default, deserialize_with = "non_empty_str")]
    camera_brand: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    camera_model: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    exposure_time: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    f_number: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    focal_length: Option<String>,
    #[serde(default, deserialize_with = "non_empty_str")]
    description: Option<String>,
}

pub(super) async fn put(
    request: Result<Json<PutImageMetadataRequest>, JsonRejection>,
    Path(image_key): Path<String>,
    Authorize(user): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<&'static str>, Error> {
    let Json(request) = request?;

    check_length(
        "fileName",
        request.file_name.as_deref(),
        super::MAXIMUM_FILE_NAME_LENGTH,
    )?;

    check_length(
        "description",
        request.description.as_deref(),
        super::MAXIMUM_DESCRIPTION_LENGTH,
    )?;

    let cimage_key = image_key.clone();
    let is_owner = state
        .db
        .call(move |conn| super::is_owner(&cimage_key, &user, conn))
        .await
        .context("Failed to update image metadata")?;

    if !is_owner {
        return Err(Error::Unathorized);
    }

    if let Some(new_name) = &request.file_name {
        let mut image_path = state.data_path.clone();
        image_path.push(&image_key);
        image_path.push("original");

        let mut entries = fs::read_dir(&image_path)
            .await
            .context("Failed to read original dir")?;

        while let Some(old_file) = entries
            .next_entry()
            .await
            .context("Failed to read entry from original dir")?
        {
            if old_file.file_name().to_str() == Some(new_name)
                || !old_file
                    .file_type()
                    .await
                    .context("Failed to check file type")?
                    .is_file()
            {
                continue;
            }

            let mut old_path = image_path.clone();
            old_path.push(old_file.file_name());
            let mut new_path = image_path.clone();
            new_path.push(&new_name);

            fs::rename(old_path, new_path)
                .await
                .context("Failed to rename original file")?;
        }
    }

    let update_str = request.update_str();
    if !update_str.is_empty() {
        // FIXME there are a bunch of errors that need to be sent to the front end here
        state
            .db
            .call(move |conn| {
                let mut params = request.update_params();
                params.push(Box::new(image_key));
                conn.execute(
                    &format!("UPDATE images SET {update_str} WHERE key = ?"),
                    rusqlite::params_from_iter(params.iter()),
                )
            })
            .await
            .context("Failed to update image metadata")?;
    } else {
        return Ok(Json("Nothing to do"));
    }

    Ok(Json("Success"))
}

impl PutImageMetadataRequest {
    fn update_str(&self) -> String {
        let mut result = Vec::new();

        if self.file_name.is_some() {
            result.push("file_name = ?");
        }

        if self.taken_at.is_some() {
            result.push("taken_at = ?");
        }

        if self.location.is_some() {
            result.push("location_latitude = ?");
            result.push("location_longitude = ?");
        }

        if self.camera_brand.is_some() {
            result.push("camera_brand = ?");
        }

        if self.exposure_time.is_some() {
            result.push("exposure_time = ?");
        }

        if self.f_number.is_some() {
            result.push("f_number = ?");
        }

        if self.focal_length.is_some() {
            result.push("focal_length = ?");
        }

        if self.description.is_some() {
            result.push("description = ?");
        }

        result.join(", ")
    }

    fn update_params(mut self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(file_name) = self.file_name.take() {
            params.push(Box::new(file_name));
        }

        if let Some(taken_at) = self.taken_at.take() {
            params.push(Box::new(taken_at));
        }

        if let Some(location) = self.location.take() {
            params.push(Box::new(location.latitude));
            params.push(Box::new(location.longitude));
        }

        if let Some(camera_brand) = self.camera_brand.take() {
            params.push(Box::new(camera_brand));
        }

        if let Some(camera_model) = self.camera_model.take() {
            params.push(Box::new(camera_model));
        }

        if let Some(exposure_time) = self.exposure_time.take() {
            params.push(Box::new(exposure_time));
        }

        if let Some(f_number) = self.f_number.take() {
            params.push(Box::new(f_number));
        }

        if let Some(focal_length) = self.focal_length.take() {
            params.push(Box::new(focal_length));
        }

        if let Some(description) = self.description.take() {
            params.push(Box::new(description));
        }

        params
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::test::{insert_image, insert_user};

    #[tokio::test]
    async fn update_metadata_description() {
        let state = AppState::in_memory_db().await;

        let (user, image) = state
            .db
            .call(move |conn| {
                let user = insert_user("test", conn).unwrap();
                let image = insert_image(&user, conn).unwrap();

                (user, image)
            })
            .await;

        let expected_description = Some(String::from("testing"));

        let description = expected_description.clone();
        let result = put(
            Ok(Json(PutImageMetadataRequest {
                description,
                ..Default::default()
            })),
            Path(image.clone()),
            Authorize(user),
            Extension(state.clone()),
        )
        .await;

        assert_eq!(result.unwrap().0, "Success");

        let metadata = state
            .db
            .call(move |conn| crate::api::image::select_image(&image, conn).unwrap())
            .await;

        assert_eq!(metadata.unwrap().description, expected_description);
    }
}
