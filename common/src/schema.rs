// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "show_format"))]
    pub struct ShowFormat;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "show_season"))]
    pub struct ShowSeason;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "show_status"))]
    pub struct ShowStatus;
}

diesel::table! {
    endings (id) {
        id -> Uuid,
        #[max_length = 255]
        show_title -> Varchar,
        number -> Int4,
        #[max_length = 255]
        title -> Varchar,
        file_name -> Varchar,
        thumbnail_file_name -> Varchar,
    }
}

diesel::table! {
    episodes (id) {
        id -> Uuid,
        #[max_length = 255]
        show_title -> Varchar,
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
        id -> Uuid,
        #[max_length = 255]
        show_title -> Varchar,
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
        id -> Uuid,
        #[max_length = 255]
        show_title -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        number -> Int4,
        file_name -> Varchar,
        thumbnail_file_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ShowFormat;
    use super::sql_types::ShowStatus;
    use super::sql_types::ShowSeason;

    shows (title) {
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        format -> Nullable<ShowFormat>,
        status -> Nullable<ShowStatus>,
        season -> Nullable<ShowSeason>,
        season_year -> Nullable<Int4>,
        image -> Nullable<Bytea>,
        banner -> Nullable<Bytea>,
    }
}

diesel::table! {
    shows_genres (id) {
        id -> Int4,
        #[max_length = 255]
        show_title -> Varchar,
        genre_name -> Varchar,
    }
}

diesel::table! {
    shows_studios (id) {
        id -> Int4,
        #[max_length = 255]
        show_title -> Varchar,
        #[max_length = 255]
        studio_name -> Varchar,
    }
}

diesel::table! {
    studios (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(endings -> shows (show_title));
diesel::joinable!(episodes -> shows (show_title));
diesel::joinable!(movies -> shows (show_title));
diesel::joinable!(openings -> shows (show_title));
diesel::joinable!(shows_genres -> genres (genre_name));
diesel::joinable!(shows_genres -> shows (show_title));
diesel::joinable!(shows_studios -> shows (show_title));
diesel::joinable!(shows_studios -> studios (studio_name));

diesel::allow_tables_to_appear_in_same_query!(
    endings,
    episodes,
    genres,
    movies,
    openings,
    shows,
    shows_genres,
    shows_studios,
    studios,
);
