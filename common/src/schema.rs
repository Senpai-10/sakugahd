// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "anime_format"))]
    pub struct AnimeFormat;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "anime_season"))]
    pub struct AnimeSeason;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "anime_status"))]
    pub struct AnimeStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AnimeFormat;
    use super::sql_types::AnimeStatus;
    use super::sql_types::AnimeSeason;

    anime (title) {
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        format -> Nullable<AnimeFormat>,
        status -> Nullable<AnimeStatus>,
        season -> Nullable<AnimeSeason>,
        season_year -> Nullable<Int4>,
        cover -> Nullable<Varchar>,
    }
}

diesel::table! {
    anime_genres (id) {
        id -> Int4,
        #[max_length = 255]
        anime_title -> Varchar,
        genre_name -> Varchar,
    }
}

diesel::table! {
    anime_studios (id) {
        id -> Int4,
        #[max_length = 255]
        anime_title -> Varchar,
        #[max_length = 255]
        studio_name -> Varchar,
    }
}

diesel::table! {
    endings (id) {
        id -> Varchar,
        #[max_length = 255]
        anime_title -> Varchar,
        number -> Int4,
        #[max_length = 255]
        title -> Varchar,
        file_name -> Varchar,
        thumbnail_file_name -> Varchar,
    }
}

diesel::table! {
    episodes (id) {
        id -> Varchar,
        #[max_length = 255]
        anime_title -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        number -> Int4,
        is_filler -> Bool,
        file_name -> Varchar,
        thumbnail_file_name -> Varchar,
    }
}

diesel::table! {
    genres (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    movies (id) {
        id -> Varchar,
        #[max_length = 255]
        anime_title -> Varchar,
        watch_after -> Int4,
        #[max_length = 255]
        title -> Varchar,
        number -> Int4,
        file_name -> Varchar,
        thumbnail_file_name -> Varchar,
    }
}

diesel::table! {
    openings (id) {
        id -> Varchar,
        #[max_length = 255]
        anime_title -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        number -> Int4,
        file_name -> Varchar,
        thumbnail_file_name -> Varchar,
    }
}

diesel::table! {
    studios (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(anime_genres -> anime (anime_title));
diesel::joinable!(anime_genres -> genres (genre_name));
diesel::joinable!(anime_studios -> anime (anime_title));
diesel::joinable!(anime_studios -> studios (studio_name));
diesel::joinable!(endings -> anime (anime_title));
diesel::joinable!(episodes -> anime (anime_title));
diesel::joinable!(movies -> anime (anime_title));
diesel::joinable!(openings -> anime (anime_title));

diesel::allow_tables_to_appear_in_same_query!(
    anime,
    anime_genres,
    anime_studios,
    endings,
    episodes,
    genres,
    movies,
    openings,
    studios,
);
