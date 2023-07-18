#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod cors;
mod db;
mod loader;
mod models;
mod routes;
mod schema;
mod thumbnail;

use db::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use loader::loader;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[get("/")]
fn home() -> String {
    String::from("Home page")
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
            eprintln!("Rocket stopped unexpectedly. (Error {})", e);
        }
    };
}
