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
}

#[derive(FromArgs, PartialEq, Debug)]
/// create account.
#[argh(subcommand, name = "add")]
pub struct AddUserArgs {
    #[argh(positional)]
    /// username
    pub username: String,
}

pub fn run_subcommand(subcommand: SubCommands, conn: &mut Connection) -> Result<(), anyhow::Error> {
    match subcommand {
        SubCommands::AddUser(args) => {
            let password =
                rpassword::prompt_password(&format!("Password for {}: ", args.username))?;

            crate::api::user::create_account(&args.username, &password, conn)?;
        }
    }

    Ok(())
}
