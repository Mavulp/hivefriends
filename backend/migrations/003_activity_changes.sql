 -- this value gets updated when draft is unset on the album
ALTER TABLE albums
RENAME COLUMN created_at TO published_at;

CREATE TABLE album_image_associations_new (
    image_key TEXT NOT NULL, -- image is a part of:
    album_key TEXT NOT NULL, -- this album
    idx INTEGER NOT NULL, -- position in the album
    created_at INTEGER NOT NULL, -- unix ts

    UNIQUE(image_key, album_key, idx),
    CONSTRAINT fk_image_key_assoc
        FOREIGN KEY (image_key)
        REFERENCES images (key)
        ON DELETE CASCADE,

    CONSTRAINT fk_album_key_assoc
        FOREIGN KEY (album_key)
        REFERENCES albums (key)
        ON DELETE CASCADE
) STRICT;

INSERT OR REPLACE INTO album_image_associations_new
SELECT
    aia.image_key,
    aia.album_key,
    aia.idx,
    (
        SELECT i.uploaded_at
        FROM images i
        JOIN album_image_associations aia2 ON aia2.image_key = i.key
        WHERE aia2.image_key = aia.image_key
        AND aia2.album_key = aia.album_key
    ) as created_at
FROM album_image_associations aia;

DROP TABLE album_image_associations;

ALTER TABLE album_image_associations_new RENAME TO album_image_associations;
