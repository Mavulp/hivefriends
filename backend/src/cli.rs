use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use anyhow::{bail, Context};
use argh::FromArgs;
use rusqlite::params;
use serde_rusqlite::from_row;
use tracing::{error, info, warn};

use crate::api::image::{orientation::ExifOrientation, DbImage};

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Top-level command.
pub struct Args {
    #[argh(subcommand)]
    pub subcommand: Option<SubCommands>,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand)]
pub enum SubCommands {
    AddUser(AddUserArgs),
    EditUser(EditUserArgs),
    ReencodeImages(ReencodeImageArgs),
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// create account.
#[argh(subcommand, name = "add")]
pub struct AddUserArgs {
    #[argh(positional)]
    /// username
    pub username: String,

    #[argh(positional)]
    /// password
    pub password: Option<String>,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// edit account.
#[argh(subcommand, name = "edit")]
pub struct EditUserArgs {
    #[argh(positional)]
    /// username
    pub username: String,

    #[argh(positional)]
    /// password
    pub password: Option<String>,
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// reencode images.
#[argh(subcommand, name = "reencode")]
pub struct ReencodeImageArgs {
    #[argh(positional)]
    /// quality
    pub quality: u8,
}

pub async fn run_subcommand(
    subcommand: SubCommands,
    db: &tokio_rusqlite::Connection,
) -> Result<(), anyhow::Error> {
    match subcommand {
        SubCommands::AddUser(args) => {
            let password = if let Some(password) = args.password {
                password
            } else {
                rpassword::prompt_password(format!("Password for {}: ", args.username))?
            };

            db.call(move |conn| crate::api::user::create_account(&args.username, &password, conn))
                .await?;
        }
        SubCommands::EditUser(args) => {
            let username = args.username.clone();
            if !db
                .call(move |conn| crate::api::user::user_exists(&username, conn))
                .await?
            {
                bail!("User {} does not exist", args.username);
            }
            let password = if let Some(password) = args.password {
                password
            } else {
                rpassword::prompt_password(format!("Password for {}: ", args.username))?
            };

            db.call(move |conn| {
                crate::api::settings::set_password(&args.username, &password, conn)
            })
            .await?;
        }
        SubCommands::ReencodeImages(args) => {
            let data_path: PathBuf = std::env::var("DATA_PATH")
                .context("DATA_PATH not set")?
                .into();

            print!("Does a backup of the data directory exist?<y/N>");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.to_lowercase().starts_with('y') {
                return Ok(());
            }

            let images = db
                .call(|conn| {
                    let mut query = conn
                        .prepare("SELECT * FROM images")
                        .context("Failed to prepare statement for images query")?;

                    let images = query
                        .query_map(params![], |row| Ok(from_row::<DbImage>(row).unwrap()))
                        .context("Failed to query user images")?
                        .collect::<Result<Vec<_>, _>>()
                        .context("Failed to collect user images")?;

                    Ok::<_, anyhow::Error>(images)
                })
                .await?;

            for image in images {
                info!("Reencoding {}", image.key);

                let mut image_path = PathBuf::from(&data_path);
                image_path.push(&image.key);

                for name in &["full.jpg", "large.jpg", "medium.jpg", "tiny.jpg"] {
                    let mut rm_path = image_path.clone();
                    rm_path.push(name);
                    std::fs::remove_file(rm_path)?;
                }

                image_path.push("original");
                image_path.push(&image.metadata.file_name);

                let mut image_data: Vec<u8> = Vec::new();
                let mut file = File::open(image_path)?;
                file.read_to_end(&mut image_data)?;

                let mut orientation = None;

                let mut bufreader = std::io::BufReader::new(Cursor::new(&*image_data));
                let exifreader = exif::Reader::new();
                match exifreader.read_from_container(&mut bufreader) {
                    Ok(exif) => {
                        orientation = crate::api::image::upload::orientation_from_exif(&exif);
                    }
                    Err(e) => {
                        warn!("Failed to read EXIF metadata: {}", e);
                    }
                }

                if let Err(e) = crate::api::image::upload::store_image(
                    data_path.clone(),
                    &image.key,
                    &image.metadata.file_name,
                    &image_data,
                    &orientation.unwrap_or(ExifOrientation::Normal),
                    args.quality,
                )
                .await
                {
                    use std::error::Error;
                    error!(
                        "Failed to reencode image {}: {}",
                        image.key,
                        e.source().unwrap()
                    );
                }
            }
        }
    }

    Ok(())
}
