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

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tag_types"))]
    pub struct TagTypes;
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
    chapters (number) {
        #[max_length = 255]
        manga_title -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        number -> Varchar,
        cover_image -> Varchar,
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
    use diesel::sql_types::*;
    use super::sql_types::TagTypes;

    genres (name) {
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> TagTypes,
    }
}

diesel::table! {
    manga (title) {
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        cover -> Nullable<Varchar>,
    }
}

diesel::table! {
    manga_genres (id) {
        id -> Int4,
        #[max_length = 255]
        manga_title -> Varchar,
        genre_name -> Varchar,
    }
}

diesel::table! {
    manga_themes (id) {
        id -> Int4,
        #[max_length = 255]
        manga_title -> Varchar,
        theme_name -> Varchar,
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
    pages (id) {
        id -> Int4,
        #[max_length = 255]
        manga_title -> Varchar,
        chapter_number -> Varchar,
        number -> Int4,
        file_name -> Varchar,
    }
}

diesel::table! {
    studios (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TagTypes;

    themes (name) {
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> TagTypes,
    }
}

diesel::joinable!(anime_genres -> anime (anime_title));
diesel::joinable!(anime_genres -> genres (genre_name));
diesel::joinable!(anime_studios -> anime (anime_title));
diesel::joinable!(anime_studios -> studios (studio_name));
diesel::joinable!(chapters -> manga (manga_title));
diesel::joinable!(endings -> anime (anime_title));
diesel::joinable!(episodes -> anime (anime_title));
diesel::joinable!(manga_genres -> genres (genre_name));
diesel::joinable!(manga_genres -> manga (manga_title));
diesel::joinable!(manga_themes -> manga (manga_title));
diesel::joinable!(manga_themes -> themes (theme_name));
diesel::joinable!(movies -> anime (anime_title));
diesel::joinable!(openings -> anime (anime_title));
diesel::joinable!(pages -> chapters (chapter_number));
diesel::joinable!(pages -> manga (manga_title));

diesel::allow_tables_to_appear_in_same_query!(
    anime,
    anime_genres,
    anime_studios,
    chapters,
    endings,
    episodes,
    genres,
    manga,
    manga_genres,
    manga_themes,
    movies,
    openings,
    pages,
    studios,
    themes,
);
