use crate::api::error::Error;
use serde::{Deserialize, Deserializer};

pub(super) fn non_empty_str<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()))
}

pub fn comma_string<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Some(
            s.split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
        ));
    }

    Ok(None)
}

pub fn check_length(
    field_name: &'static str,
    field: Option<&str>,
    maximum_length: u64,
) -> Result<(), Error> {
    if let Some(field) = field {
        let field = &field;
        if field.len() as u64 > maximum_length {
            return Err(Error::TooManyCharacters {
                field: field_name,
                maximum_length,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
pub mod test {
    use crate::api::{
        album::{self, InsertAlbum, InsertShareToken},
        comment, image, user,
    };
    use rusqlite_migration::Migrations;

    pub async fn setup_database() -> tokio_rusqlite::Connection {
        let db = tokio_rusqlite::Connection::open_in_memory().await.unwrap();

        let migrations = Migrations::new(crate::MIGRATIONS.to_vec());

        db.call(move |conn| {
            conn.pragma_update(None, "foreign_keys", "OFF")?;
            migrations.to_latest(conn)?;
            conn.pragma_update(None, "foreign_keys", "ON")?;

            Ok::<_, anyhow::Error>(())
        })
        .await
        .unwrap();

        db
    }

    pub fn insert_user(name: &str, conn: &rusqlite::Connection) -> String {
        user::insert(name, "", 0, conn).unwrap();

        name.to_string()
    }

    pub fn insert_alias(name: &str, content: &str, conn: &rusqlite::Connection) {
        conn.execute(
            "INSERT INTO aliases (name, content) VALUES (?1, ?2)",
            rusqlite::params![name, content],
        )
        .unwrap();
    }

    pub fn insert_image(uploader: &str, conn: &rusqlite::Connection) -> String {
        let key = blob_uuid::random_blob();

        let i = image::DbImage {
            key: key.clone(),
            uploader: uploader.to_string(),
            uploaded_at: 0,

            ..Default::default()
        };

        image::insert(&i, conn).unwrap();

        key
    }

    pub fn insert_comment(
        author: &str,
        image_key: &str,
        album_key: &str,
        text: &str,
        conn: &rusqlite::Connection,
    ) -> comment::Comment {
        comment::insert_comment(
            author.into(),
            text.into(),
            image_key.into(),
            album_key.into(),
            0,
            conn,
        )
        .unwrap()
    }

    pub fn insert_album(album: InsertAlbum, conn: &rusqlite::Connection) -> String {
        fn insert_album<'a>(key: &'a str, mut album: InsertAlbum<'a>, conn: &rusqlite::Connection) {
            album.key = key;
            album::insert_album(album, conn).unwrap();
        }

        let key = blob_uuid::random_blob();
        insert_album(&key, album, conn);

        key
    }

    pub fn insert_share_token(rows: InsertShareToken, conn: &rusqlite::Connection) -> String {
        fn insert_token<'a>(
            token: &'a str,
            mut rows: InsertShareToken<'a>,
            conn: &rusqlite::Connection,
        ) {
            rows.share_token = token;
            album::insert_share_token(rows, conn).unwrap();
        }

        let token = blob_uuid::random_blob();
        insert_token(&token, rows, conn);

        token
    }
}
