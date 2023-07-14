#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod db;
mod loader;
mod models;
mod schema;

use db::establish_connection;
use diesel::prelude::*;
use loader::loader;
use models::show::Show;
use rocket_seek_stream::SeekStream;
use schema::shows::dsl::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[get("/")]
fn home() -> String {
    String::from("Home page")
}

#[get("/video/<file_name>")]
fn video<'a>(file_name: String) -> std::io::Result<SeekStream<'a>> {
    let video_path = format!(
        "/run/media/senpai/Toshiba external hard drive/anime/Bleach (English SUB)/{}",
        file_name
    );

    SeekStream::from_path(video_path)
}

#[rocket::main]
async fn main() {
    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    loader(&mut connection);

    let listshows = shows
        .load::<Show>(&mut connection)
        .expect("Error loading shows");

    // println!("Listing shows:");
    // for show in listshows {
    //     println!("{:?}", show);
    // }

    // match rocket::build()
    //     .mount("/api", routes![home, video])
    //     .launch()
    //     .await
    // {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("Rocket stopped unexpectedly. (Error {})", e);
    //     }
    // };
}
