-- junction table for shows and studios
CREATE TABLE IF NOT EXISTS shows_studios (
    id SERIAL PRIMARY KEY NOT NULL,
    show_id UUID NOT NULL REFERENCES shows(id),
    studio_name VARCHAR(255) NOT NULL REFERENCES studios(name)
);
