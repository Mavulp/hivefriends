use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get_service, Router},
    Extension,
};
use rusqlite::params;
use rusqlite_migration::{Migrations, M};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use tracing::*;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub struct AppState {
    db: tokio_rusqlite::Connection,
    data_path: PathBuf,
    image_quality: u8,
}

#[cfg(test)]
impl AppState {
    pub(crate) async fn in_memory_db() -> Arc<AppState> {
        let data_path = tempdir::TempDir::new("hivefriends-test-data")
            .unwrap()
            .path()
            .to_path_buf();
        let db = util::test::setup_database().await;

        Arc::new(AppState {
            db,
            data_path,
            image_quality: 0,
        })
    }
}

pub mod cli;
pub mod util;

pub mod api {
    pub mod activity;
    pub mod album;
    pub mod alias;
    pub mod auth;
    pub mod comment;
    pub mod error;
    pub mod image;
    pub mod login;
    pub mod public_auth;
    pub mod settings;
    pub mod user;
}

const AUTH_TIME_SECONDS: u64 = 3600 * 24 * 30;

pub fn api_route(db: tokio_rusqlite::Connection, data_path: PathBuf, image_quality: u8) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        .nest("/api/auth", api::auth::api_route())
        .nest("/api/login", api::login::api_route())
        .nest("/api/activity", api::activity::api_route())
        .nest("/api/comments", api::comment::api_route())
        .nest("/api/public/comments", api::comment::public_api_route())
        .nest("/api/images", api::image::api_route())
        .nest("/api/albums", api::album::api_route())
        .nest("/api/public/albums", api::album::public_api_route())
        .nest("/api/users", api::user::api_route())
        .nest("/api/aliases", api::alias::api_route())
        .nest("/api/settings", api::settings::api_route())
        .nest(
            "/data/image",
            get_service(ServeDir::new(data_path.clone())).handle_error(handle_error),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(AppState {
            db,
            data_path,
            image_quality,
        })))
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub(crate) const MIGRATIONS: [M; 4] = [
    M::up(include_str!("../migrations/001_initial.sql")),
    M::up(include_str!(
        "../migrations/002_username_collate_nocase.sql"
    ))
    .foreign_key_check(),
    M::up(include_str!("../migrations/003_activity_changes.sql")).foreign_key_check(),
    M::up(include_str!("../migrations/004_add_iso.sql")),
];

pub async fn setup_database(path: &Path) -> anyhow::Result<tokio_rusqlite::Connection> {
    let db = tokio_rusqlite::Connection::open(path).await?;

    let migrations = Migrations::new(MIGRATIONS.to_vec());

    db.call(move |conn| {
        conn.pragma_update(None, "foreign_keys", "OFF")?;
        migrations.to_latest(conn)?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        Ok::<_, anyhow::Error>(())
    })
    .await?;

    info!("Clearing old auth sessions");
    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();
    let oldest_auth_time = now - Duration::from_secs(AUTH_TIME_SECONDS);
    db.call(move |conn| {
        conn.execute(
            "DELETE FROM auth_sessions WHERE created_at<?1",
            params![oldest_auth_time.as_secs()],
        )
    })
    .await?;

    Ok(db)
}
