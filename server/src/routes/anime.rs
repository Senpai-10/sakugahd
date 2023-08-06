use crate::db::establish_connection;
use common::models::anime::Anime;
use common::schema;
use diesel::prelude::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use std::path::Path;
use urlencoding::decode;

#[get("/anime?<limit>")]
pub fn anime(limit: Option<i64>) -> Json<Vec<Anime>> {
    let mut conn = establish_connection();
    let mut query = schema::anime::table.into_boxed();

    query = query.order(schema::anime::title);

    if limit.is_some() {
        let l = limit.unwrap();
        query = query.limit(l);
    }

    Json(query.load(&mut conn).expect("Can't load anime"))
}

#[get("/anime/<title>")]
pub fn anime_one(title: String) -> Json<Anime> {
    let mut conn = establish_connection();

    Json(
        schema::anime::dsl::anime
            .filter(schema::anime::title.eq(&title))
            .first(&mut conn)
            .expect("Can't load anime"),
    )
}

#[get("/anime/<title>/genres")]
pub fn anime_genres(title: String) -> Json<Vec<String>> {
    let mut conn = establish_connection();

    Json(
        schema::anime_genres::dsl::anime_genres
            .filter(schema::anime_genres::anime_title.eq(&title))
            .select(schema::anime_genres::genre_name)
            .load(&mut conn)
            .expect("Can't load genres"),
    )
}

#[get("/anime/<title>/studios")]
pub fn anime_studios(title: String) -> Json<Vec<String>> {
    let mut conn = establish_connection();

    Json(
        schema::anime_studios::dsl::anime_studios
            .filter(schema::anime_studios::anime_title.eq(&title))
            .select(schema::anime_studios::studio_name)
            .load(&mut conn)
            .expect("Can't load studios"),
    )
}

#[get("/anime/<title>/cover/<file_name>")]
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
