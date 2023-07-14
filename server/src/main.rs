#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod db;
mod loader;
mod models;
mod schema;

use self::schema::episodes;
use db::establish_connection;
use diesel::prelude::*;
use diesel::QueryDsl;
use loader::loader;
use models::episode::Episode;
use models::show::Show;
use rocket::serde::uuid::Uuid;
use rocket_seek_stream::SeekStream;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use schema::shows;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[get("/")]
fn home() -> String {
    String::from("Home page")
}

fn get_video_absolute_path(id: Uuid, anime_directory: String) -> String {
    let mut conn = establish_connection();
    let abs_path = std::path::Path::new(&anime_directory);

    let e = episodes::dsl::episodes
        .filter(episodes::id.eq(&id))
        .first::<Episode>(&mut conn);

    if e.is_ok() {
        let e = e.unwrap();

        let file_name: String = e.file_name;
        let show_id = e.show_id;

        let s: Show = shows::dsl::shows
            .filter(shows::id.eq(&show_id))
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

    return String::new();
}

#[get("/video/<id>")]
fn video<'a>(id: Uuid) -> std::io::Result<SeekStream<'a>> {
    // let ep = episodes::dsl::episodes.find(id).find(&conn);

    let file_path = get_video_absolute_path(id, std::env::var("ANIME_DIRECTORY").unwrap());

    SeekStream::from_path(file_path)
}

#[rocket::main]
async fn main() {
    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    loader(&mut connection);

    match rocket::build()
        .mount("/api", routes![home, video])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Rocket stopped unexpectedly. (Error {})", e);
        }
    };
}
