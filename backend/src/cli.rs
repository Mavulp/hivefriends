use anyhow::bail;
use argh::FromArgs;
use rusqlite::Connection;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct Args {
    #[argh(subcommand)]
    pub subcommand: Option<SubCommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommands {
    AddUser(AddUserArgs),
    EditUser(EditUserArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
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

#[derive(FromArgs, PartialEq, Debug)]
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

pub fn run_subcommand(subcommand: SubCommands, conn: &mut Connection) -> Result<(), anyhow::Error> {
    match subcommand {
        SubCommands::AddUser(args) => {
            let password = if let Some(password) = args.password {
                password
            } else {
                rpassword::prompt_password(&format!("Password for {}: ", args.username))?
            };

            crate::api::user::create_account(&args.username, &password, conn)?;
        }
        SubCommands::EditUser(args) => {
            if !crate::api::user::user_exists(&args.username, conn)? {
                bail!("User {} does not exist", args.username);
            }
            let password = if let Some(password) = args.password {
                password
            } else {
                rpassword::prompt_password(&format!("Password for {}: ", args.username))?
            };

            crate::api::settings::set_password(&args.username, &password, conn)?;
        }
    }

    Ok(())
}
