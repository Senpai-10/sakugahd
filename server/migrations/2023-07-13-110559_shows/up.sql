DO $$ BEGIN
    CREATE TYPE show_format AS ENUM ('TV', 'OVA', 'ONA', 'MOVIE', 'SPECIAL');
    CREATE TYPE show_status AS ENUM ('FINISHED', 'ONGOING');
    CREATE TYPE show_season AS ENUM ('SPRING', 'SUMMER', 'FALL', 'WINTER');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS shows (
    id UUID PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    format show_format,
    status show_status,
    season show_season,
    season_year INT,
    -- example: bleach
    directory_name VARCHAR NOT NULL,
    image BYTEA,
    banner BYTEA
);
