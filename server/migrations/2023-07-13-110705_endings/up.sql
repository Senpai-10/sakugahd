CREATE TABLE IF NOT EXISTS endings (
    id UUID PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title),
    number INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);
