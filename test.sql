CREATE TYPE relation_type AS ENUM ('SEQUEL', 'PREQUEL', 'ALTERNATIVE_SETTING',
                                    'ALTERNATIVE_VERSION', 'SIDE_STORY', 'SUMMARY',
                                    'FULL_STORY', 'PARENT_STORY', 'SPIN_OFF', 'OTHER');

CREATE TABLE IF NOT anime_related (
    id serial PRIMARY KEY NOT NULL,
    anime_id VARCHAR NOT NULL,
    relative_id VARCHAR NOT NULL,
    type relation_type NOT NULL
);

