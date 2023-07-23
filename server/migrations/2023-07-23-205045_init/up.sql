DO $$ BEGIN
    CREATE TYPE show_format AS ENUM ('TV', 'OVA', 'ONA', 'MOVIE', 'SPECIAL');
    CREATE TYPE show_status AS ENUM ('FINISHED', 'ONGOING');
    CREATE TYPE show_season AS ENUM ('SPRING', 'SUMMER', 'FALL', 'WINTER');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS shows (
    title VARCHAR(255) PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    format show_format,
    status show_status,
    season show_season,
    season_year INT,
    cover VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS studios (
    name VARCHAR(255) PRIMARY KEY NOT NULL
);

-- junction table for shows and studios
CREATE TABLE IF NOT EXISTS shows_studios (
    id SERIAL PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title) ON DELETE CASCADE,
    studio_name VARCHAR(255) NOT NULL REFERENCES studios(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS genres (
    name VARCHAR(255) NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS shows_genres (
    id SERIAL PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title) ON DELETE CASCADE,
    genre_name VARCHAR NOT NULL REFERENCES genres(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS episodes (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    -- episode number
    number INT NOT NULL,
    is_filler BOOLEAN NOT NULL,
    -- later join the anime_directory_path with 'ep' and file_name to stream the video
    -- result = 'path/to/anime/show_name/ep/bleach 1.mp4'
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS movies (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title) ON DELETE CASCADE,
    -- watch movie after n number of episodes
    watch_after INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS openings (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS endings (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title) ON DELETE CASCADE,
    number INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);
