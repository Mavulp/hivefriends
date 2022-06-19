use anyhow::Context;
use tracing::*;

use hivefriends::{api_route, cli, setup_database};

use std::net::SocketAddr;
use std::path::PathBuf;

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

    let db_path: PathBuf = std::env::var("DB_PATH").context("DB_PATH not set")?.into();
    let db = setup_database(&db_path).await?;

    if let Some(sub) = args.subcommand {
        return db.call(move |conn| cli::run_subcommand(sub, conn)).await;
    }

    let data_path = std::env::var("DATA_PATH")
        .context("DATA_PATH not set")?
        .into();

    let bind_addr: SocketAddr = std::env::var("BIND_ADDRESS")
        .context("BIND_ADDRESS not set")?
        .parse()
        .context("BIND_ADDRESS could not be parsed")?;

    info!("listening on {}", bind_addr);
    axum::Server::try_bind(&bind_addr)?
        .serve(api_route(db, data_path).into_make_service())
        .await
        .unwrap();

    Ok(())
}
