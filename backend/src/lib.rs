use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get_service, Router},
    Extension,
};
use deadpool_sqlite::{Config, Pool, Runtime};
use rusqlite::params;
use rusqlite_migration::{Migrations, M};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::*;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub type DbPool<M> = deadpool::managed::Pool<M, deadpool::managed::Object<M>>;
pub type FileDb = deadpool_sqlite::Manager;

pub trait SqliteDatabase:
    deadpool::managed::Manager<Type = Self::T, Error = rusqlite::Error>
{
    type T: DbInteractable;
}

#[async_trait::async_trait]
pub trait DbInteractable {
    async fn interact<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut rusqlite::Connection) -> R + Send + 'static,
        R: Send + 'static;
}

#[async_trait::async_trait]
impl DbInteractable for deadpool_sync::SyncWrapper<rusqlite::Connection> {
    async fn interact<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut rusqlite::Connection) -> R + Send + 'static,
        R: Send + 'static,
    {
        self.interact(|conn| f(conn)).await.unwrap()
    }
}

impl SqliteDatabase for deadpool_sqlite::Manager {
    type T = deadpool_sync::SyncWrapper<rusqlite::Connection>;
}

pub struct AppState<M: SqliteDatabase = FileDb> {
    pool: DbPool<M>,
    data_path: PathBuf,
}

#[cfg(test)]
impl AppState {
    pub(crate) async fn in_memory_db() -> Arc<AppState<util::test::InMemorySqliteManager>> {
        let data_path = tempdir::TempDir::new("hivefriends-test-data")
            .unwrap()
            .path()
            .to_path_buf();
        let pool = util::test::setup_database().await.unwrap();

        Arc::new(AppState { pool, data_path })
    }
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

const AUTH_TIME_SECONDS: u64 = 3600 * 24 * 7;

pub fn api_route(pool: Pool, data_path: PathBuf) -> Router {
    Router::new()
        .nest("/api/auth", api::auth::api_route())
        .nest("/api/login", api::login::api_route())
        .nest("/api/images/", api::image::api_route())
        .nest("/api/albums/", api::album::api_route())
        .nest("/api/users/", api::user::api_route())
        .nest("/api/settings/", api::settings::api_route())
        .nest(
            "/data/image/",
            get_service(ServeDir::new(data_path.clone())).handle_error(handle_error),
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(AppState { pool, data_path })))
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub(crate) const MIGRATIONS: [M; 1] = [M::up(include_str!("../migrations/001_initial.sql"))];

pub async fn setup_database(path: &Path) -> anyhow::Result<Pool> {
    let cfg = Config::new(path);
    let pool = cfg.create_pool(Runtime::Tokio1)?;

    let migrations = Migrations::new(MIGRATIONS.to_vec());

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
