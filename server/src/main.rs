#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod cors;
mod db;
mod loader;
mod routes;

use db::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use loader::Loader;
use std::path::Path;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[get("/")]
fn home() -> String {
    String::from("Home page")
}

#[rocket::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    let env_anime_directory =
        std::env::var("ANIME_DIRECTORY").expect("ANIME_DIRECTORY must be set");
    let anime_directory = Path::new(&env_anime_directory);

    Loader::new(anime_directory, &mut connection).run();

    match rocket::build()
        .attach(cors::Cors)
        .mount(
            "/api",
            routes![
                home,
                routes::video::video_episodes,
                routes::video::video_movies,
                routes::video::video_openings,
                routes::video::video_endings,
                routes::shows::shows,
                routes::episodes::episodes,
                routes::movies::movies,
                routes::openings::openings,
                routes::endings::endings,
            ],
        )
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            error!("Rocket stopped unexpectedly. (Error {})", e);
        }
    };
}
