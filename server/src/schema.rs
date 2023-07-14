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
        show_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        file_name -> Varchar,
        thumbnail -> Nullable<Bytea>,
    }
}

diesel::table! {
    episodes (id) {
        id -> Uuid,
        show_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        number -> Int4,
        is_filler -> Bool,
        file_name -> Varchar,
        thumbnail -> Nullable<Bytea>,
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
        show_id -> Uuid,
        watch_after -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        file_name -> Varchar,
        thumbnail -> Nullable<Bytea>,
    }
}

diesel::table! {
    openings (id) {
        id -> Uuid,
        show_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        file_name -> Varchar,
        thumbnail -> Nullable<Bytea>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ShowFormat;
    use super::sql_types::ShowStatus;
    use super::sql_types::ShowSeason;

    shows (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        format -> Nullable<ShowFormat>,
        status -> Nullable<ShowStatus>,
        season -> Nullable<ShowSeason>,
        season_year -> Nullable<Int4>,
        directory_name -> Varchar,
        image -> Nullable<Bytea>,
        banner -> Nullable<Bytea>,
    }
}

diesel::table! {
    shows_genres (id) {
        id -> Int4,
        show_id -> Uuid,
        genre_name -> Varchar,
    }
}

diesel::table! {
    shows_studios (id) {
        id -> Int4,
        show_id -> Uuid,
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

diesel::joinable!(endings -> shows (show_id));
diesel::joinable!(episodes -> shows (show_id));
diesel::joinable!(movies -> shows (show_id));
diesel::joinable!(openings -> shows (show_id));
diesel::joinable!(shows_genres -> genres (genre_name));
diesel::joinable!(shows_genres -> shows (show_id));
diesel::joinable!(shows_studios -> shows (show_id));
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
