
-- A lot of tables were missing `COLLATE NOCASE` on the username which already
-- caused some bugs around tagged users in albums (user_album_associations) to
-- propagate from the front end down to the back end. UNIQUE should work
-- correctly again on all occurrence of username.

-- Additionally this adds a missing foreign key constraint to the author of
-- comments since that was missing.

CREATE TABLE images_new (
    key TEXT PRIMARY KEY NOT NULL, -- used for filesystem path and in API responses

    -- metadata
    file_name TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    taken_at INTEGER NULL, -- unix ts
    location_latitude TEXT NULL, -- gps data
    location_longitude TEXT NULL, -- gps data
    camera_brand TEXT NULL,
    camera_model TEXT NULL,
    exposure_time TEXT NULL,
    f_number TEXT NULL,
    focal_length TEXT NULL,

    description TEXT NULL,
    uploader TEXT NOT NULL COLLATE NOCASE,
    uploaded_at INTEGER NOT NULL, -- time since unix epoch (unix ts)

    CONSTRAINT fk_uploader_assoc
        FOREIGN KEY (uploader)
        REFERENCES users (username)
        ON DELETE CASCADE
) STRICT;

INSERT OR REPLACE INTO images_new
SELECT
    i.key,
    i.file_name,
    i.size_bytes,
    i.taken_at,
    i.location_latitude,
    i.location_longitude,
    i.camera_brand,
    i.camera_model,
    i.exposure_time,
    i.f_number,
    i.focal_length,
    i.description,
    (SELECT u.username FROM users u where u.username = i.uploader) as uploader,
    i.uploaded_at
FROM images i;

DROP TABLE images;

ALTER TABLE images_new RENAME TO images;

----

CREATE TABLE comments_new (
    id INTEGER PRIMARY KEY NOT NULL,
    author TEXT NOT NULL COLLATE NOCASE,
    album_key TEXT NOT NULL,
    image_key TEXT NOT NULL,
    created_at INTEGER NOT NULL, -- unix ts

    text TEXT NOT NULL,

    CONSTRAINT fk_author_assoc
        FOREIGN KEY (author)
        REFERENCES users (username)
        ON DELETE CASCADE,

    CONSTRAINT fk_album_key_assoc
        FOREIGN KEY (album_key)
        REFERENCES albums (key)
        ON DELETE CASCADE,

    CONSTRAINT fk_image_key_assoc
        FOREIGN KEY (image_key)
        REFERENCES images (key)
        ON DELETE CASCADE
) STRICT;

INSERT OR REPLACE INTO comments_new
SELECT
    c.id,
    (SELECT u.username FROM users u where u.username = c.author) as author,
    c.album_key,
    c.image_key,
    c.created_at,
    c.text
FROM comments c;

DROP TABLE comments;

ALTER TABLE comments_new RENAME TO comments;

----

CREATE TABLE albums_new (
    key TEXT PRIMARY KEY NOT NULL, -- used in API responses
    title TEXT NOT NULL,
    description TEXT NULL,
    cover_key TEXT NOT NULL,
    author TEXT NOT NULL COLLATE NOCASE,
    draft INTEGER NOT NULL DEFAULT 0, -- boolean

    -- timeframe of the event covered by this album
    timeframe_from INTEGER NULL, -- unix ts
    timeframe_to INTEGER NULL, -- unix ts

    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_author_assoc
        FOREIGN KEY (author)
        REFERENCES users (username)
        ON DELETE CASCADE

    CONSTRAINT fk_cover_key_assoc
        FOREIGN KEY (cover_key)
        REFERENCES images (key)
) STRICT;

INSERT OR REPLACE INTO albums_new
SELECT
    a.key,
    a.title,
    a.description,
    a.cover_key,
    (SELECT u.username FROM users u where u.username = a.author) as author,
    a.draft,
    a.timeframe_from,
    a.timeframe_to,
    a.created_at
FROM albums a;

DROP TABLE albums;

ALTER TABLE albums_new RENAME TO albums;

----

CREATE TABLE auth_sessions_new (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL COLLATE NOCASE,
    token TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_username_assoc
        FOREIGN KEY (username)
        REFERENCES users (username)
        ON DELETE CASCADE
) STRICT;

INSERT OR REPLACE INTO auth_sessions_new
SELECT
    s.id,
    (SELECT u.username FROM users u where u.username = s.username),
    s.token,
    s.created_at
FROM auth_sessions s;

DROP TABLE auth_sessions;

ALTER TABLE auth_sessions_new RENAME TO auth_sessions;

----

-- tags users that are in the album
CREATE TABLE user_album_associations_new (
    username TEXT NOT NULL COLLATE NOCASE, -- user is tagged in:
    album_key TEXT NOT NULL, -- this album

    UNIQUE(username, album_key),

    CONSTRAINT fk_username_assoc
        FOREIGN KEY (username)
        REFERENCES users (username)
        ON DELETE CASCADE,

    CONSTRAINT fk_album_key_assoc
        FOREIGN KEY (album_key)
        REFERENCES albums (key)
        ON DELETE CASCADE
) STRICT;

INSERT OR REPLACE INTO user_album_associations_new
SELECT
    (SELECT u.username FROM users u where u.username = uaa.username),
    uaa.album_key
FROM user_album_associations uaa;

DROP TABLE user_album_associations;

ALTER TABLE user_album_associations_new RENAME TO user_album_associations;

----

CREATE TABLE album_share_tokens_new (
    share_token TEXT PRIMARY KEY NOT NULL,
    album_key TEXT NOT NULL, -- this album
    created_by TEXT NOT NULL COLLATE NOCASE,
    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_album_key_assoc
        FOREIGN KEY (album_key)
        REFERENCES albums (key)
        ON DELETE CASCADE

    CONSTRAINT fk_created_by_assoc
        FOREIGN KEY (created_by)
        REFERENCES users (username)
        ON DELETE CASCADE
) STRICT;

INSERT OR REPLACE INTO album_share_tokens_new
SELECT
    ast.share_token,
    ast.album_key,
    (SELECT u.username FROM users u where u.username = ast.created_by),
    ast.created_at
FROM album_share_tokens ast;

DROP TABLE album_share_tokens;

ALTER TABLE album_share_tokens_new RENAME TO album_share_tokens;
