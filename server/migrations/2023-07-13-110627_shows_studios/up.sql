-- junction table for shows and studios
CREATE TABLE IF NOT EXISTS shows_studios (
    id SERIAL PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title),
    studio_name VARCHAR(255) NOT NULL REFERENCES studios(name)
);
