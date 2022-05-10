use argh::FromArgs;
use rusqlite::{params, Connection};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;

use std::time::SystemTime;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct Args {
    #[argh(subcommand)]
    subcommand: SubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Account(AccountArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
/// create account.
#[argh(subcommand, name = "create-account")]
struct AccountArgs {
    #[argh(option)]
    /// username
    username: String,

    #[argh(option)]
    /// username
    password: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "two")]
struct SubCommandTwo {
    #[argh(switch)]
    /// whether to fooey
    fooey: bool,
}

fn main() {
    dotenv::dotenv().ok();

    let args: Args = argh::from_env();

    let db_path = std::env::var("DB_PATH").expect("DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    match args.subcommand {
        SubCommands::Account(args) => create_account(args, conn),
    }
}

fn create_account(args: AccountArgs, conn: Connection) {
   let salt = SaltString::generate(&mut OsRng);
   let argon2 = Argon2::default();
   let phc_string = argon2
       .hash_password(args.password.as_bytes(), &salt).unwrap()
       .to_string(); 
    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;

   conn.execute(
       r"INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
       params![args.username, phc_string, now]).unwrap();
}
