CREATE TYPE show_type AS ENUM ('TV', 'OVA', 'ONA', 'MOVIE', 'SPECIAL');
CREATE TYPE show_status AS ENUM ('FINISHED', 'ONGOING');
CREATE TYPE show_season AS ENUM ('SPRING', 'SUMMER', 'FALL', 'WINTER');

CREATE TABLE shows (
    id VARCHAR PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    type show_type NOT NULL,
    status show_status NOT NULL,
    season show_season NOT NULL,
    season_year INT NOT NULL,
    -- example: bleach
    folder_name VARCHAR NOT NULL,
    image BYTEA,
    banner BYTEA
);

CREATE TABLE studios (
    name VARCHAR(255) PRIMARY KEY NOT NULL
);

-- junction table for shows and studios
CREATE TABLE shows_studios (
    show_id VARCHAR NOT NULL REFERENCES shows(id),
    studio_name VARCHAR(255) NOT NULL REFERENCES studios(name)
);


CREATE TABLE genres (
    title VARCHAR(255) PRIMARY KEY NOT NULL
);

-- junction table for shows and genres
CREATE TABLE shows_genres (
    show_id VARCHAR NOT NULL REFERENCES shows(id),
    genre_title VARCHAR NOT NULL REFERENCES genres(title)
);

CREATE TABLE episodes (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_id VARCHAR NOT NULL REFERENCES shows(id),
    title VARCHAR(255) NOT NULL,
    is_filler BOOLEAN NOT NULL,
    -- later join the anime_folder_path with 'ep' and file_name to stream the video
    -- result = 'path/to/anime/show_name/ep/bleach 1.mp4'
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);

CREATE TABLE movies (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_id VARCHAR NOT NULL REFERENCES shows(id),
    -- watch movie after n number of episodes
    watch_after INT,
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);

CREATE TABLE openings (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_id VARCHAR NOT NULL REFERENCES shows(id),
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);

CREATE TABLE endings (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_id VARCHAR NOT NULL REFERENCES shows(id),
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);

