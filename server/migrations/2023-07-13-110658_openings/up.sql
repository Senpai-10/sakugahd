CREATE TABLE IF NOT EXISTS openings (
    id VARCHAR PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title),
    title VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail_file_name VARCHAR NOT NULL
);
