CREATE TABLE images (
    key TEXT PRIMARY KEY NOT NULL, -- used for filesystem path and in API responses

    -- metadata
    file_name TEXT NULL,
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
    uploader TEXT NOT NULL,
    uploaded_at INTEGER NOT NULL, -- time since unix epoch (unix ts)

    CONSTRAINT fk_uploader_assoc
        FOREIGN KEY (uploader)
        REFERENCES users (username)
        ON DELETE CASCADE
) STRICT;

CREATE TABLE comments (
    id INTEGER PRIMARY KEY NOT NULL,
    author TEXT NOT NULL,
    image_key TEXT NOT NULL,
    created_at INTEGER NOT NULL, -- unix ts

    text TEXT NOT NULL,

    CONSTRAINT fk_image_key_assoc
        FOREIGN KEY (image_key)
        REFERENCES images (key)
        ON DELETE CASCADE
) STRICT;

CREATE TABLE albums (
    key TEXT PRIMARY KEY NOT NULL, -- used in API responses
    title TEXT NOT NULL,
    description TEXT NULL,
    cover_key TEXT NOT NULL,
    locations TEXT NULL,
    author TEXT NOT NULL,
    draft INTEGER NOT NULL DEFAULT 0, -- boolean

    -- timeframe of the event covered by this album
    timeframe_from INTEGER NULL, -- unix ts
    timeframe_to INTEGER NULL, -- unix ts

    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_uploader_assoc
        FOREIGN KEY (author)
        REFERENCES users (username)
        ON DELETE CASCADE

    CONSTRAINT fk_cover_key_assoc
        FOREIGN KEY (cover_key)
        REFERENCES images (key)
) STRICT;

CREATE TABLE album_image_associations (
    image_key TEXT NOT NULL, -- image is a part of:
    album_key TEXT NOT NULL, -- this album

    CONSTRAINT fk_image_key_assoc
        FOREIGN KEY (image_key)
        REFERENCES images (key)
        ON DELETE CASCADE,

    CONSTRAINT fk_album_key_assoc
        FOREIGN KEY (album_key)
        REFERENCES albums (key)
        ON DELETE CASCADE
) STRICT;

CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NULL UNIQUE,
    bio TEXT NULL,
    avatar_key TEXT NULL,
    banner_key TEXT NULL,
    accent_color TEXT NOT NULL DEFAULT '229, 125, 35',
    featured_album_key TEXT NULL,
    created_at INTEGER NOT NULL, -- unix ts

    -- private
    password_hash TEXT NOT NULL,
    color_theme TEXT NOT NULL DEFAULT 'light-theme',

    CONSTRAINT fk_avatar_key_assoc
        FOREIGN KEY (avatar_key)
        REFERENCES images (key),

    CONSTRAINT fk_banner_key_assoc
        FOREIGN KEY (banner_key)
        REFERENCES images (key),

    CONSTRAINT fk_featured_album_key_assoc
        FOREIGN KEY (featured_album_key)
        REFERENCES albums (key)
) STRICT;

CREATE TABLE auth_sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    token TEXT NOT NULL,
    created_at INTEGER NOT NULL, -- unix ts

    CONSTRAINT fk_username_assoc
        FOREIGN KEY (username)
        REFERENCES users (username)
        ON DELETE CASCADE
) STRICT;

-- tags users that are in the album
CREATE TABLE user_album_associations (
    username TEXT NOT NULL, -- user is tagged in:
    album_key TEXT NOT NULL, -- this album

    CONSTRAINT fk_username_assoc
        FOREIGN KEY (username)
        REFERENCES users (username)
        ON DELETE CASCADE,

    CONSTRAINT fk_album_key_assoc
        FOREIGN KEY (album_key)
        REFERENCES albums (key)
        ON DELETE CASCADE
) STRICT;
