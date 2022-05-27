use serde::Deserializer;

pub(super) fn non_empty_str<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()))
}

#[cfg(test)]
pub mod test {
    use crate::api::{
        album::{self, InsertAlbum, InsertShareToken},
        comment, image, user,
    };
    use crate::DbInteractable;
    use deadpool::{
        async_trait,
        managed::{self, Object, Pool, PoolBuilder},
    };
    use rusqlite_migration::Migrations;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub struct InMemorySqliteManager(Arc<Mutex<rusqlite::Connection>>);

    pub struct InMemoryWrapper(Arc<Mutex<rusqlite::Connection>>);

    #[async_trait::async_trait]
    impl DbInteractable for InMemoryWrapper {
        async fn interact<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&mut rusqlite::Connection) -> R + Send + 'static,
            R: Send + 'static,
        {
            let mut conn = self.0.lock().await;

            f(&mut *conn)
        }
    }

    #[async_trait]
    impl managed::Manager for InMemorySqliteManager {
        type Type = InMemoryWrapper;
        type Error = rusqlite::Error;

        async fn create(&self) -> Result<Self::Type, Self::Error> {
            Ok(InMemoryWrapper(self.0.clone()))
        }

        async fn recycle(&self, _conn: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
            Ok(())
        }
    }

    pub type TestPool = crate::DbPool<InMemorySqliteManager>;
    pub type TestPoolBuilder = PoolBuilder<InMemorySqliteManager, Object<InMemorySqliteManager>>;

    impl crate::SqliteDatabase for InMemorySqliteManager {
        type T = InMemoryWrapper;
    }

    pub async fn setup_database() -> anyhow::Result<TestPool> {
        let conn = Arc::new(Mutex::new(rusqlite::Connection::open_in_memory()?));
        let pool: TestPool = Pool::builder(InMemorySqliteManager(conn)).build()?;

        let migrations = Migrations::new(crate::MIGRATIONS.to_vec());

        let conn = pool.get().await?;
        conn.interact(move |conn| migrations.to_latest(conn))
            .await?;

        Ok(pool)
    }

    pub fn insert_user(name: &str, conn: &rusqlite::Connection) -> anyhow::Result<String> {
        user::insert(name, "", 0, conn)?;

        Ok(name.to_string())
    }

    pub fn insert_image(uploader: &str, conn: &rusqlite::Connection) -> anyhow::Result<String> {
        let key = blob_uuid::random_blob();

        let i = image::DbImageMetadata {
            key: key.clone(),
            uploader: uploader.to_string(),
            size_bytes: 0,
            uploaded_at: 0,

            ..Default::default()
        };

        image::insert(&i, conn)?;

        Ok(key)
    }

    pub fn insert_comment(
        author: &str,
        image_key: &str,
        album_key: &str,
        text: &str,
        conn: &rusqlite::Connection,
    ) -> anyhow::Result<comment::Comment> {
        comment::insert_comment(
            author.into(),
            text.into(),
            image_key.into(),
            album_key.into(),
            0,
            conn,
        )
    }

    pub fn insert_album(album: InsertAlbum, conn: &rusqlite::Connection) -> anyhow::Result<String> {
        fn insert_album<'a>(
            key: &'a str,
            mut album: InsertAlbum<'a>,
            conn: &rusqlite::Connection,
        ) -> anyhow::Result<()> {
            album.key = &key;
            album::insert_album(album, conn)?;

            Ok(())
        }

        let key = blob_uuid::random_blob();
        insert_album(&key, album, conn)?;

        Ok(key)
    }

    pub fn insert_share_token(
        rows: InsertShareToken,
        conn: &rusqlite::Connection,
    ) -> anyhow::Result<String> {
        fn insert_token<'a>(
            token: &'a str,
            mut rows: InsertShareToken<'a>,
            conn: &rusqlite::Connection,
        ) -> anyhow::Result<()> {
            rows.share_token = &token;
            album::insert_share_token(rows, conn)?;

            Ok(())
        }

        let token = blob_uuid::random_blob();
        insert_token(&token, rows, conn)?;

        Ok(token)
    }
}
