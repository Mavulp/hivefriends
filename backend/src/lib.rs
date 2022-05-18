use axum::{routing::Router, Extension};
use deadpool_sqlite::{Config, Pool, Runtime};
use rusqlite::params;
use rusqlite_migration::{Migrations, M};
use tower_http::trace::TraceLayer;
use tracing::*;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub struct AppState {
    pool: Pool,
    data_path: PathBuf,
}

pub mod cli;
pub mod util;

pub mod api {
    pub mod album;
    pub mod auth;
    pub mod error;
    pub mod image;
    pub mod login;
    pub mod settings;
    pub mod user;
}

pub mod data {
    pub mod image;
}

const AUTH_TIME_SECONDS: u64 = 3600 * 24 * 7;

pub fn api_route(pool: Pool, data_path: PathBuf) -> Router {
    Router::new()
        .nest("/api/auth", api::auth::api_route())
        .nest("/api/login", api::login::api_route())
        .nest("/api/images/", api::image::api_route())
        .nest("/api/albums/", api::album::api_route())
        .nest("/api/users/", api::user::api_route())
        .nest("/api/settings/", api::settings::api_route())
        .nest("/data/image/", data::image::api_route())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(AppState { pool, data_path })))
}

pub async fn setup_database(path: &Path) -> anyhow::Result<Pool> {
    let cfg = Config::new(path);
    let pool = cfg.create_pool(Runtime::Tokio1)?;

    let migrations = Migrations::new(vec![M::up(include_str!("../migrations/001_initial.sql"))]);

    let conn = pool.get().await?;
    conn.interact(move |conn| migrations.to_latest(conn))
        .await
        .unwrap()?;

    info!("Clearing old auth sessions");
    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();
    let oldest_auth_time = now - Duration::from_secs(AUTH_TIME_SECONDS);
    conn.interact(move |conn| {
        conn.execute(
            "DELETE FROM auth_sessions WHERE created_at<?1",
            params![oldest_auth_time.as_secs()],
        )
    })
    .await
    .unwrap()?;

    Ok(pool)
}
