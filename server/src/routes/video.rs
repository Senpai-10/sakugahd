use crate::db::establish_connection;
use common::models::ending::Ending;
use common::models::episode::Episode;
use common::models::movie::Movie;
use common::models::opening::Opening;
use common::schema;
use diesel::prelude::*;
use diesel::QueryDsl;
use rocket_seek_stream::SeekStream;

#[get("/anime/<title>/episodes/<number>")]
pub fn video_episodes<'a>(title: String, number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    if let Ok(v) = schema::episodes::dsl::episodes
        .filter(schema::episodes::anime_title.eq(&title))
        .filter(schema::episodes::number.eq(&number))
        .first::<Episode>(&mut conn)
    {
        let file_name: String = v.file_name;

        file_path = String::from(
            abs_path
                .join(title)
                .join("episodes")
                .join(file_name)
                .to_str()
                .unwrap(),
        )
    };

    SeekStream::from_path(file_path)
}

#[get("/anime/<title>/movies/<number>")]
pub fn video_movies<'a>(title: String, number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    if let Ok(v) = schema::movies::dsl::movies
        .filter(schema::movies::anime_title.eq(&title))
        .filter(schema::movies::number.eq(&number))
        .first::<Movie>(&mut conn)
    {
        let file_name: String = v.file_name;

        file_path = String::from(
            abs_path
                .join(title)
                .join("movies")
                .join(file_name)
                .to_str()
                .unwrap(),
        )
    };

    SeekStream::from_path(file_path)
}

#[get("/anime/<title>/openings/<number>")]
pub fn video_openings<'a>(title: String, number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    if let Ok(v) = schema::openings::dsl::openings
        .filter(schema::openings::anime_title.eq(&title))
        .filter(schema::openings::number.eq(&number))
        .first::<Opening>(&mut conn)
    {
        let file_name: String = v.file_name;

        file_path = String::from(
            abs_path
                .join(title)
                .join("openings")
                .join(file_name)
                .to_str()
                .unwrap(),
        )
    };

    SeekStream::from_path(file_path)
}

#[get("/anime/<title>/endings/<number>")]
pub fn video_endings<'a>(title: String, number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    if let Ok(v) = schema::endings::dsl::endings
        .filter(schema::endings::anime_title.eq(&title))
        .filter(schema::endings::number.eq(&number))
        .first::<Ending>(&mut conn)
    {
        let file_name: String = v.file_name;

        file_path = String::from(
            abs_path
                .join(title)
                .join("endings")
                .join(file_name)
                .to_str()
                .unwrap(),
        )
    };

    SeekStream::from_path(file_path)
}
