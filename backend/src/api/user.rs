use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::Path, routing::get, Extension, Json, Router};
use rand::rngs::OsRng;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;

use std::sync::Arc;
use std::time::SystemTime;

use crate::api::{auth::Authorize, error::Error};
use crate::AppState;

pub fn api_route() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/:username", get(get_user_by_username))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_key: Option<String>,
    pub banner_key: Option<String>,
    pub accent_color: Option<String>,
    pub featured_album_key: Option<String>,
    pub country: Option<String>,
    pub met: Vec<String>,
    pub albums_uploaded: Vec<String>,
    pub created_at: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DbUser {
    username: String,
    display_name: Option<String>,
    bio: Option<String>,
    avatar_key: Option<String>,
    banner_key: Option<String>,
    accent_color: Option<String>,
    featured_album_key: Option<String>,
    country: Option<String>,
    created_at: i64,
}

async fn get_users(
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>, Error> {
    state
        .db
        .call(move |conn| {
            let query = "SELECT \
                username, \
                display_name, \
                bio, \
                avatar_key, \
                banner_key, \
                accent_color, \
                featured_album_key, \
                country, \
                created_at \
                FROM users"
                .to_string();

            let params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

            let mut stmt = conn
                .prepare(&query)
                .context("Failed to prepare statement for album query")?;
            let db_users = stmt
                .query_map(rusqlite::params_from_iter(params.iter()), |row| {
                    Ok(from_row::<DbUser>(row).unwrap())
                })
                .context("Failed to query images")?
                .collect::<Result<Vec<_>, _>>()
                .context("Failed to collect albums")?;

            let mut users = Vec::new();
            for db_user in db_users {
                let mut stmt = conn
                    .prepare(
                        "SELECT a.\"key\" FROM albums a \
                    WHERE a.author = ?1 \
                    AND a.draft == false",
                    )
                    .context("Failed to prepare user albums query")?;
                let album_key_iter = stmt
                    .query_map(params![db_user.username], |row| {
                        Ok(from_row::<String>(row).unwrap())
                    })
                    .context("Failed to query user albums")?;

                let albums_uploaded = album_key_iter
                    .collect::<Result<Vec<_>, _>>()
                    .context("Failed to collect albums uploaded")?;

                let mut stmt = conn
                    .prepare(
                        "SELECT u2.username FROM users u1 \
                        INNER JOIN user_album_associations uaa ON uaa.username = u1.username \
                        INNER JOIN albums a ON a.key = uaa.album_key \
                        INNER JOIN user_album_associations uaa2 ON uaa2.album_key = a.key \
                        INNER JOIN users u2 ON uaa2.username = u2.username \
                        WHERE u1.username = ?1 \
                        AND u2.username != ?1 \
                        AND a.draft == false
                        GROUP BY u2.username",
                    )
                    .context("Failed to prepare met users query")?;
                let met_iter = stmt
                    .query_map(params![db_user.username], |row| {
                        Ok(from_row::<String>(row).unwrap())
                    })
                    .context("Failed to query met users")?;

                let met = met_iter
                    .collect::<Result<Vec<_>, _>>()
                    .context("Failed to collect met users")?;

                users.push(User {
                    username: db_user.username,
                    display_name: db_user.display_name,
                    avatar_key: db_user.avatar_key,
                    banner_key: db_user.banner_key,
                    accent_color: db_user.accent_color,
                    featured_album_key: db_user.featured_album_key,
                    country: db_user.country,
                    bio: db_user.bio,
                    met,
                    albums_uploaded,
                    created_at: db_user.created_at,
                });
            }
            Ok(Json(users))
        })
        .await
}

async fn get_user_by_username(
    Path(username): Path<String>,
    Authorize(_): Authorize,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<User>, Error> {
    let user = state
        .db
        .call(move |conn| {
            conn.query_row(
                "SELECT
                    username, \
                    display_name, \
                    bio, \
                    avatar_key, \
                    banner_key, \
                    accent_color, \
                    featured_album_key, \
                    country, \
                    created_at \
                FROM users WHERE username = ?1",
                params![username],
                |row| Ok(from_row::<DbUser>(row).unwrap()),
            )
            .optional()
        })
        .await
        .context("Failed to query users")?;

    if let Some(db_user) = user {
        let cusername = db_user.username.clone();
        let albums_uploaded = state
            .db
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT a.\"key\" FROM albums a \
                    WHERE a.author = ?1 \
                    AND a.draft == false",
                )?;
                let album_iter = stmt.query_map(params![cusername], |row| {
                    Ok(from_row::<String>(row).unwrap())
                })?;

                album_iter.collect::<Result<Vec<_>, _>>()
            })
            .await
            .context("Failed to query albums uploaded")?;

        let cusername = db_user.username.clone();
        let met = state
            .db
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT u2.username FROM users u1 \
                    INNER JOIN user_album_associations uaa ON uaa.username = u1.username \
                    INNER JOIN albums a ON a.key = uaa.album_key \
                    INNER JOIN user_album_associations uaa2 ON uaa2.album_key = a.key \
                    INNER JOIN users u2 ON uaa2.username = u2.username \
                    WHERE u1.username = ?1 \
                    AND u2.username != ?1 \
                    AND a.draft == false
                    GROUP BY u2.username",
                )?;
                let album_iter = stmt.query_map(params![cusername], |row| {
                    Ok(from_row::<String>(row).unwrap())
                })?;

                album_iter.collect::<Result<Vec<_>, _>>()
            })
            .await
            .context("Failed to query met users")?;

        Ok(Json(User {
            username: db_user.username,
            display_name: db_user.display_name,
            bio: db_user.bio,
            avatar_key: db_user.avatar_key,
            banner_key: db_user.banner_key,
            accent_color: db_user.accent_color,
            featured_album_key: db_user.featured_album_key,
            country: db_user.country,
            met,
            albums_uploaded,
            created_at: db_user.created_at,
        }))
    } else {
        Err(Error::NotFound)
    }
}

pub fn create_account(username: &str, password: &str, conn: &mut Connection) -> anyhow::Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let phc_string = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let now = SystemTime::UNIX_EPOCH.elapsed()?.as_secs();

    insert(username, &phc_string, now, conn)?;

    Ok(())
}

pub fn insert(username: &str, phc_string: &str, now: u64, conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
        params![username, phc_string, now],
    )?;

    Ok(())
}

pub fn user_exists(username: &str, conn: &Connection) -> anyhow::Result<bool> {
    let result = conn.query_row(
        "SELECT 1 FROM users WHERE username = ?1",
        params![username],
        |_| Ok(()),
    );

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        Ok(false)
    } else {
        result?;

        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::album::InsertAlbum;
    use crate::util::test::{insert_album, insert_image, insert_user};
    use assert_matches::assert_matches;

    #[tokio::test]
    async fn get_by_username() {
        let state = AppState::in_memory_db().await;

        let users = state
            .db
            .call(move |conn| {
                let users = vec![
                    insert_user("test", conn).unwrap(),
                    insert_user("test2", conn).unwrap(),
                ];
                let user = users[0].clone();
                let image = insert_image(&user, conn).unwrap();

                // Insert 2 albums to ensure we have a working GROUP BY on the met users list
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                users
            })
            .await;

        let result = get_user_by_username(
            Path(users[0].clone()),
            Authorize("".into()),
            Extension(state),
        )
        .await;

        assert_matches!(result, Ok(Json(user)) => {
            assert_matches!(&user.met[..], [met] => {
                assert_eq!(met, &users[1]);
            })
        });
    }

    #[tokio::test]
    async fn get_all() {
        let state = AppState::in_memory_db().await;

        let _ = state
            .db
            .call(move |conn| {
                let users = vec![
                    insert_user("test", conn).unwrap(),
                    insert_user("test2", conn).unwrap(),
                ];
                let user = users[0].clone();
                let image = insert_image(&user, conn).unwrap();

                // Insert 2 albums to ensure we have a working GROUP BY on the met users list
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();
                let _ = insert_album(
                    InsertAlbum {
                        cover_key: &image,
                        image_keys: &[image.clone()],
                        tagged_users: &users,
                        author: &user,
                        ..Default::default()
                    },
                    conn,
                )
                .unwrap();

                users
            })
            .await;

        let result = get_users(Authorize("".into()), Extension(state)).await;

        assert_matches!(result, Ok(Json(users)) => {
            assert_matches!(&users[..], [user1, user2] => {
                assert_matches!(&user1.met[..], [met] => {
                    assert_eq!(met, &user2.username);
                });
                assert_matches!(&user2.met[..], [met] => {
                    assert_eq!(met, &user1.username);
                });
            });
        });
    }
}
