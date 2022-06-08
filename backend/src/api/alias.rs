use anyhow::Context;

use axum::{routing::get, Router};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::api::error::Error;
use crate::{AppState, DbInteractable, FileDb, SqliteDatabase};

use super::auth::Authorize;
use std::sync::Arc;

use serde_rusqlite::from_row;

use rusqlite::params;

pub fn api_route() -> Router {
    Router::new().route("/", get(get_aliases::<FileDb>))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alias {
    name: String,
    content: String,
}

async fn get_aliases<D: SqliteDatabase>(
    Authorize(_username): Authorize,
    Extension(state): Extension<Arc<AppState<D>>>,
) -> Result<Json<Vec<Alias>>, Error> {
    let conn = state.pool.get().await.context("Failed to get connection")?;

    conn.interact(move |conn| {
        let mut query = conn
            .prepare("SELECT * FROM ALIASES")
            .context("Failed to prepare statement for aliases query")?;

        let dbdata = query
            .query_map(params![], |row| Ok(from_row::<Alias>(row).unwrap()))
            .context("Failed to query aliases")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect aliases")?;

        Ok(Json(dbdata))
    })
    .await
}
