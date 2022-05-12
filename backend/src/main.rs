use anyhow::Context;
use axum::{routing::Router, Extension};
use deadpool_sqlite::{Config, Pool, Runtime};
use rusqlite_migration::{Migrations, M};
use tower_http::trace::TraceLayer;
use tracing::*;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

pub struct AppState {
    pool: Pool,
    data_path: PathBuf,
}

pub mod cli;

pub mod api {
    pub mod album;
    pub mod auth;
    pub mod error;
    pub mod image;
    pub mod login;
    pub mod user;
}

pub mod data {
    pub mod image;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    tracing_subscriber::fmt::init();

    info!("Running {}", app_name);

    if let Err(e) = run().await {
        let err = e
            .chain()
            .skip(1)
            .fold(e.to_string(), |acc, cause| format!("{}: {}\n", acc, cause));
        error!("{}", err);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let args: cli::Args = argh::from_env();

    let db_path = std::env::var("DB_PATH").context("DB_PATH not set")?;
    let pool = setup_database(&db_path).await?;

    if let Some(sub) = args.subcommand {
        let conn = pool.get().await.context("Failed to get connection")?;
        return conn
            .interact(move |conn| cli::run_subcommand(sub, conn))
            .await
            .unwrap();
    }

    let data_path = std::env::var("DATA_PATH")
        .context("DATA_PATH not set")?
        .into();

    let bind_addr: SocketAddr = std::env::var("BIND_ADDRESS")
        .context("BIND_ADDRESS not set")?
        .parse()
        .context("BIND_ADDRESS could not be parsed")?;

    let app = Router::new()
        .nest("/api/login", api::login::api_route())
        .nest("/api/images/", api::image::api_route())
        .nest("/api/albums/", api::album::api_route())
        .nest("/api/users/", api::user::api_route())
        .nest("/data/image/", data::image::api_route())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(AppState { pool, data_path })));

    info!("listening on {}", bind_addr);
    axum::Server::try_bind(&bind_addr)?
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn setup_database(path: &str) -> anyhow::Result<Pool> {
    let cfg = Config::new(path);
    let pool = cfg.create_pool(Runtime::Tokio1)?;

    let migrations = Migrations::new(vec![M::up(include_str!("../migrations/001_initial.sql"))]);

    let conn = pool.get().await?;
    conn.interact(move |conn| migrations.to_latest(conn))
        .await
        .unwrap()?;

    Ok(pool)
}
