CREATE TABLE IF NOT EXISTS shows_genres (
    id SERIAL PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title),
    genre_name VARCHAR NOT NULL REFERENCES genres(name)
);
