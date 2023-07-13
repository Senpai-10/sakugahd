CREATE TABLE IF NOT EXISTS episodes (
    id UUID PRIMARY KEY NOT NULL,
    show_id UUID NOT NULL REFERENCES shows(id),
    title VARCHAR(255) NOT NULL,
    -- episode number
    number INT NOT NULL,
    is_filler BOOLEAN NOT NULL,
    -- later join the anime_folder_path with 'ep' and file_name to stream the video
    -- result = 'path/to/anime/show_name/ep/bleach 1.mp4'
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA NOT NULL
);
