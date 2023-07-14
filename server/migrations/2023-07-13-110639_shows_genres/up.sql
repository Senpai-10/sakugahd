CREATE TABLE IF NOT EXISTS shows_genres (
    id SERIAL PRIMARY KEY NOT NULL,
    show_id UUID NOT NULL REFERENCES shows(id),
    genre_name VARCHAR NOT NULL REFERENCES genres(name)
);
