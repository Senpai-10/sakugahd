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
            routes![home, routes::video::video, routes::shows::shows],
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
