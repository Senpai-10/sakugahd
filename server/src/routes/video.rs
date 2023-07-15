use crate::db::establish_connection;
use crate::models::ending::Ending;
use crate::models::episode::Episode;
use crate::models::movie::Movie;
use crate::models::opening::Opening;
use crate::models::show::Show;
use crate::schema;
use diesel::prelude::*;
use diesel::QueryDsl;
use rocket::serde::uuid::Uuid;
use rocket_seek_stream::SeekStream;

#[get("/video/<id>")]
pub fn video<'a>(id: Uuid) -> std::io::Result<SeekStream<'a>> {
    let file_path = get_video_absolute_path(id, std::env::var("ANIME_DIRECTORY").unwrap());

    SeekStream::from_path(file_path)
}

fn get_video_absolute_path(id: Uuid, anime_directory: String) -> String {
    let mut conn = establish_connection();
    let abs_path = std::path::Path::new(&anime_directory);

    match schema::episodes::dsl::episodes
        .filter(schema::episodes::id.eq(&id))
        .first::<Episode>(&mut conn)
    {
        Ok(e) => {
            let file_name: String = e.file_name;
            let show_id = e.show_id;

            let s: Show = schema::shows::dsl::shows
                .filter(schema::shows::id.eq(&show_id))
                .first::<Show>(&mut conn)
                .unwrap();

            return String::from(
                abs_path
                    .join(s.directory_name)
                    .join("episodes")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            );
        }
        Err(_) => {}
    };

    match schema::movies::dsl::movies
        .filter(schema::movies::id.eq(&id))
        .first::<Movie>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;
            let show_id = v.show_id;

            let s: Show = schema::shows::dsl::shows
                .filter(schema::shows::id.eq(&show_id))
                .first::<Show>(&mut conn)
                .unwrap();

            return String::from(
                abs_path
                    .join(s.directory_name)
                    .join("movies")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            );
        }
        Err(_) => {}
    };

    match schema::openings::dsl::openings
        .filter(schema::openings::id.eq(&id))
        .first::<Opening>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;
            let show_id = v.show_id;

            let s: Show = schema::shows::dsl::shows
                .filter(schema::shows::id.eq(&show_id))
                .first::<Show>(&mut conn)
                .unwrap();

            return String::from(
                abs_path
                    .join(s.directory_name)
                    .join("openings")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            );
        }
        Err(_) => {}
    };

    match schema::endings::dsl::endings
        .filter(schema::endings::id.eq(&id))
        .first::<Ending>(&mut conn)
    {
        Ok(v) => {
            let file_name: String = v.file_name;
            let show_id = v.show_id;

            let s: Show = schema::shows::dsl::shows
                .filter(schema::shows::id.eq(&show_id))
                .first::<Show>(&mut conn)
                .unwrap();

            return String::from(
                abs_path
                    .join(s.directory_name)
                    .join("endings")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
            );
        }
        Err(_) => {}
    };

    return String::new();
}
