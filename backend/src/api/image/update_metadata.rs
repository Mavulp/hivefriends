use anyhow::Context;
use axum::{
    extract::{rejection::JsonRejection, Path},
    Extension, Json,
};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::api::{auth::Authorize, error::Error};
use crate::util::non_empty_str;
use crate::{AppState, DbInteractable, SqliteDatabase};

use super::Location;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PutImageMetadataRequest {
    #[serde(default, deserialize_with = "non_empty_str")]
    file_name: Option<String>,
    taken_at: Option<u64>,
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

pub(super) async fn put<D: SqliteDatabase>(
    request: Result<Json<PutImageMetadataRequest>, JsonRejection>,
    Path(image_key): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<&'static str>, Error> {
    let Json(request) = request?;
    let conn = state.pool.get().await.context("Failed to get connection")?;

    let update_str = request.update_str();
    if !update_str.is_empty() {
        // FIXME there are a bunch of errors that need to be sent to the front end here
        conn
            .interact(move |conn| {
                let mut params = request.update_params();
                params.push(Box::new(image_key));
                conn.execute(
                    &format!("UPDATE images SET {update_str} WHERE key = ?"),
                    rusqlite::params_from_iter(params.iter()),
                )
            })
            .await.context("Failed to update image metadata")?;
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

        let conn = state.pool.get().await.unwrap();

        let result: anyhow::Result<_> = conn
            .interact(move |conn| {
                let user = insert_user("test", conn)?;
                let image = insert_image(&user, conn)?;

                Ok((user, image))
            })
            .await;
        let (user, image) = result.unwrap();

        let expected_description = Some(String::from("testing"));

        let description = expected_description.clone();
        let result = put(
            Ok(Json(PutImageMetadataRequest {
                description,
                ..Default::default()
            })),
            Path(image.clone()),
            Authorize(user),
            Extension(state),
        )
        .await;

        assert_eq!(result.unwrap().0, "Success");

        let result: anyhow::Result<_> = conn
            .interact(move |conn| {
                let metadata = crate::api::image::select_image(&image, conn)?;

                Ok(metadata)
            })
            .await;

        assert_eq!(result.unwrap().unwrap().description, expected_description);
    }
}
