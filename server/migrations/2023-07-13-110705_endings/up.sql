CREATE TABLE IF NOT EXISTS endings (
    id UUID PRIMARY KEY NOT NULL,
    show_id UUID NOT NULL REFERENCES shows(id),
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);
