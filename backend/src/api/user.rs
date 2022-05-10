use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;
use rusqlite::{params, Connection};

use std::time::SystemTime;

pub fn create_account(
    username: &str,
    password: &str,
    conn: &mut Connection,
) -> anyhow::Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let phc_string = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs() as u32;

    conn.execute(
        r"INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
        params![username, phc_string, now],
    )?;

    Ok(())
}
