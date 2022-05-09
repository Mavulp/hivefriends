CREATE TABLE image (
    id INTEGER PRIMARY KEY,
    key TEXT NOT NULL, -- used for filesystem path and in API responses
    created_at INTEGER NOT NULL -- time since unix epoch
) STRICT;

CREATE INDEX image_key_idx
ON image (key);

CREATE TABLE album (
    id INTEGER PRIMARY KEY,
    key TEXT NOT NULL, -- used in API responses
    created_at INTEGER NOT NULL -- time since unix epoch
) STRICT;

CREATE INDEX album_key_idx
ON album (key);

CREATE TABLE album_image_association (
    image_id INTEGER NOT NULL, -- image is a part of:
    album_id INTEGER NOT NULL, -- this album

    CONSTRAINT fk_image_id_assoc
        FOREIGN KEY (image_id)
        REFERENCES image (id)
        ON DELETE CASCADE,
        
    CONSTRAINT fk_album_id_assoc
        FOREIGN KEY (album_id)
        REFERENCES album (id)
        ON DELETE CASCADE
) STRICT;
