use crate::db::establish_connection;
use crate::models::ending::Ending;
use crate::models::episode::Episode;
use crate::models::movie::Movie;
use crate::models::opening::Opening;
use crate::schema;
use diesel::prelude::*;
use diesel::QueryDsl;
use rocket_seek_stream::SeekStream;

#[get("/shows/<title>/episodes/<ep_number>")]
pub fn video_episodes<'a>(title: String, ep_number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    match schema::episodes::dsl::episodes
        .filter(schema::episodes::show_title.eq(&title))
        .filter(schema::episodes::number.eq(&ep_number))
        .first::<Episode>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;

            file_path = String::from(
                abs_path
                    .join(title)
                    .join("episodes")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            )
        }
        Err(_) => {}
    };

    SeekStream::from_path(file_path)
}

#[get("/shows/<title>/movies/<ep_number>")]
pub fn video_movies<'a>(title: String, ep_number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    match schema::movies::dsl::movies
        .filter(schema::movies::show_title.eq(&title))
        .filter(schema::movies::number.eq(&ep_number))
        .first::<Movie>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;

            file_path = String::from(
                abs_path
                    .join(title)
                    .join("movies")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            )
        }
        Err(_) => {}
    };

    SeekStream::from_path(file_path)
}

#[get("/shows/<title>/openings/<ep_number>")]
pub fn video_openings<'a>(title: String, ep_number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    match schema::openings::dsl::openings
        .filter(schema::openings::show_title.eq(&title))
        .filter(schema::openings::number.eq(&ep_number))
        .first::<Opening>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;

            file_path = String::from(
                abs_path
                    .join(title)
                    .join("openings")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            )
        }
        Err(_) => {}
    };

    SeekStream::from_path(file_path)
}

#[get("/shows/<title>/endings/<ep_number>")]
pub fn video_endings<'a>(title: String, ep_number: i32) -> std::io::Result<SeekStream<'a>> {
    let mut conn = establish_connection();
    let anime_directory = std::env::var("ANIME_DIRECTORY").unwrap();
    let abs_path = std::path::Path::new(&anime_directory);
    let mut file_path = String::new();

    match schema::endings::dsl::endings
        .filter(schema::endings::show_title.eq(&title))
        .filter(schema::endings::number.eq(&ep_number))
        .first::<Ending>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;

            file_path = String::from(
                abs_path
                    .join(title)
                    .join("endings")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            )
        }
        Err(_) => {}
    };

    SeekStream::from_path(file_path)
}
