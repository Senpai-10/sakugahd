#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod cors;
mod db;
mod loader;
mod models;
mod schema;

use db::establish_connection;
use diesel::prelude::*;
use diesel::QueryDsl;
use loader::loader;
use models::ending::Ending;
use models::episode::Episode;
use models::movie::Movie;
use models::opening::Opening;
use models::show::Show;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket_seek_stream::SeekStream;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[get("/")]
fn home() -> String {
    String::from("Home page")
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

#[get("/video/<id>")]
fn video<'a>(id: Uuid) -> std::io::Result<SeekStream<'a>> {
    let file_path = get_video_absolute_path(id, std::env::var("ANIME_DIRECTORY").unwrap());

    SeekStream::from_path(file_path)
}

#[get("/shows")]
fn shows() -> Json<Vec<Show>> {
    let mut conn = establish_connection();

    Json(
        schema::shows::dsl::shows
            .load(&mut conn)
            .expect("Can't load shows"),
    )
}

#[rocket::main]
async fn main() {
    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    loader(&mut connection);

    match rocket::build()
        .attach(cors::Cors)
        .mount("/api", routes![home, video, shows])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Rocket stopped unexpectedly. (Error {})", e);
        }
    };
}
