CREATE TYPE anime_format AS ENUM ('TV', 'OVA', 'ONA', 'MOVIE', 'SPECIAL');
CREATE TYPE anime_status AS ENUM ('FINISHED', 'ONGOING');
CREATE TYPE anime_season AS ENUM ('SPRING', 'SUMMER', 'FALL', 'WINTER');

CREATE TABLE anime (
    title VARCHAR(255) PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    format anime_format,
    status anime_status,
    season anime_season,
    season_year INT,
    cover VARCHAR
);

CREATE TABLE studios (
    name VARCHAR(255) PRIMARY KEY NOT NULL
);

-- junction table for anime and studios
CREATE TABLE anime_studios (
    id SERIAL PRIMARY KEY NOT NULL,
    anime_title VARCHAR(255) NOT NULL REFERENCES anime(title) ON DELETE CASCADE,
    studio_name VARCHAR(255) NOT NULL REFERENCES studios(name) ON DELETE CASCADE
);

CREATE TABLE genres (
    name VARCHAR(255) NOT NULL PRIMARY KEY
);

CREATE TABLE anime_genres (
    id SERIAL PRIMARY KEY NOT NULL,
    anime_title VARCHAR(255) NOT NULL REFERENCES anime(title) ON DELETE CASCADE,
    genre_name VARCHAR NOT NULL REFERENCES genres(name) ON DELETE CASCADE
);

CREATE TABLE episodes (
    id VARCHAR PRIMARY KEY NOT NULL,
    anime_title VARCHAR(255) NOT NULL REFERENCES anime(title) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    -- episode number
    number INT NOT NULL,
    is_filler BOOLEAN NOT NULL,
    -- later join the anime_directory_path with 'ep' and file_name to stream the video
    -- result = 'path/to/anime/anime_name/ep/bleach 1.mp4'
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);

CREATE TABLE movies (
    id VARCHAR PRIMARY KEY NOT NULL,
    anime_title VARCHAR(255) NOT NULL REFERENCES anime(title) ON DELETE CASCADE,
    -- watch movie after n number of episodes
    watch_after INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);

CREATE TABLE openings (
    id VARCHAR PRIMARY KEY NOT NULL,
    anime_title VARCHAR(255) NOT NULL REFERENCES anime(title) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);

CREATE TABLE endings (
    id VARCHAR PRIMARY KEY NOT NULL,
    anime_title VARCHAR(255) NOT NULL REFERENCES anime(title) ON DELETE CASCADE,
    number INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);
