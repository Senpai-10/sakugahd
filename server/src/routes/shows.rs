use crate::db::establish_connection;
use common::models::show::Show;
use common::schema;
use diesel::prelude::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use std::path::Path;
use urlencoding::decode;

#[get("/shows")]
pub fn shows() -> Json<Vec<Show>> {
    let mut conn = establish_connection();

    Json(
        schema::shows::dsl::shows
            .load(&mut conn)
            .expect("Can't load shows"),
    )
}

#[get("/shows/<title>")]
pub fn show(title: String) -> Json<Show> {
    let mut conn = establish_connection();

    Json(
        schema::shows::dsl::shows
            .filter(schema::shows::title.eq(&title))
            .first(&mut conn)
            .expect("Can't load shows"),
    )
}

#[get("/shows/<title>/cover/<file_name>")]
pub async fn get_cover(title: String, file_name: String) -> NamedFile {
    let decoded_title: String = decode(&title).expect("UTF-8").to_string();
    let decoded_file_name: String = decode(&file_name).expect("UTF-8").to_string();

    let env_anime_directory =
        std::env::var("ANIME_DIRECTORY").expect("ANIME_DIRECTORY must be set");
    let anime_directory = Path::new(&env_anime_directory);

    let file_path = Path::new(&anime_directory)
        .join(decoded_title)
        .join(decoded_file_name);
    NamedFile::open(&file_path)
        .await
        .expect("Failed to get cover image")
}
