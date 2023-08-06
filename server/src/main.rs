#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod cors;
mod db;
mod loaders;
mod routes;

use db::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use loaders::anime::AnimeLoader;
use loaders::manga::MangaLoader;
use rocket::fs::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("/usr/src/sakugahd-dist/").join(file))
        .await
        .ok()
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("/usr/src/sakugahd-dist/index.html").await
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
    let env_manga_directory =
        std::env::var("MANGA_DIRECTORY").expect("MANGA_DIRECTORY must be set");
    let manga_directory = Path::new(&env_manga_directory);

    AnimeLoader::new(anime_directory, &mut connection).run();
    MangaLoader::new(manga_directory, &mut connection).run();

    match rocket::build()
        .attach(cors::Cors)
        .mount("/", routes![index, files])
        .mount(
            "/api",
            routes![
                routes::manga::manga,
                routes::manga::manga_one,
                routes::manga::manga_genres,
                routes::manga::manga_themes,
                routes::manga::get_cover,
                routes::manga::manga_chapters,
                routes::manga::manga_chapter_pages,
                routes::manga::manga_page,
                routes::video::video_episodes,
                routes::video::video_movies,
                routes::video::video_openings,
                routes::video::video_endings,
                routes::anime::anime_one,
                routes::anime::anime_studios,
                routes::anime::anime_genres,
                routes::anime::anime,
                routes::anime::get_cover,
                routes::studios::studios,
                routes::studios::studio,
                routes::genres::genres,
                routes::genres::genre,
                routes::episodes::episodes,
                routes::movies::movies,
                routes::openings::openings,
                routes::endings::endings,
                routes::thumbnail::thumbnail
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
