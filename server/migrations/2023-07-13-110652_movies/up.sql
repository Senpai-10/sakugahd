CREATE TABLE IF NOT EXISTS movies (
    id UUID PRIMARY KEY NOT NULL,
    show_title VARCHAR(255) NOT NULL REFERENCES shows(title),
    -- watch movie after n number of episodes
    watch_after INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    number INT NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA
);
