CREATE TABLE IF NOT EXISTS movies (
    id UUID PRIMARY KEY NOT NULL,
    show_id UUID NOT NULL REFERENCES shows(id),
    -- watch movie after n number of episodes
    watch_after INT,
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR NOT NULL,
    thumbnail BYTEA NOT NULL
);
