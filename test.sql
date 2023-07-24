CREATE TYPE relation_type AS ENUM ('SEQUEL', 'PREQUEL', 'ALTERNATIVE_SETTING',
                                    'ALTERNATIVE_VERSION', 'SIDE_STORY', 'SUMMARY',
                                    'FULL_STORY', 'PARENT_STORY', 'SPIN_OFF', 'OTHER');

CREATE TABLE anime_related (
    -- The relationship is between anime and relative
    --  if type == 'SEQUEL' that means anime is the SEQUEL of relative
    id serial PRIMARY KEY NOT NULL,
    anime_title VARCHAR NOT NULL,
    relative_title VARCHAR NOT NULL,
    type relation_type NOT NULL
);

/*

What to do when inserting to this table
    If type == 'SEQUEL'
        relative is the PREQUEL of anime
    IF type == 'PREQUEL'
        relative is the SEQUEL of anime

    TODO: fill this later

*/
