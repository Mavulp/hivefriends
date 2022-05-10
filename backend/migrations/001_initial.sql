CREATE TABLE images (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL, -- used for filesystem path and in API responses
    created_at INTEGER NOT NULL -- time since unix epoch
) STRICT;

CREATE INDEX images_key_idx
ON images (key);

CREATE TABLE albums (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL, -- used in API responses
    created_at INTEGER NOT NULL -- time since unix epoch
) STRICT;

CREATE INDEX albums_key_idx
ON albums (key);

CREATE TABLE albums_images_associations (
    image_id INTEGER NOT NULL, -- image is a part of:
    album_id INTEGER NOT NULL, -- this album

    CONSTRAINT fk_image_id_assoc
        FOREIGN KEY (image_id)
        REFERENCES images (id)
        ON DELETE CASCADE,
        
    CONSTRAINT fk_album_id_assoc
        FOREIGN KEY (album_id)
        REFERENCES albums (id)
        ON DELETE CASCADE
) STRICT;
