use crate::db::establish_connection;
use common::models::anime::Anime;
use common::models::anime_studio::AnimeStudio;
use common::models::studio::Studio;
use common::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/studios")]
pub fn studios() -> Json<Vec<String>> {
    let mut conn = establish_connection();

    Json(
        schema::studios::dsl::studios
            .select(schema::studios::name)
            .order(schema::studios::dsl::name)
            .load(&mut conn)
            .expect("Can't load openings"),
    )
}

#[get("/studios/<name>")]
pub fn studio(name: String) -> Json<Vec<Anime>> {
    let mut conn = establish_connection();

    let studio_name = schema::studios::table
        .filter(schema::studios::dsl::name.eq(&name))
        .select(Studio::as_select())
        .get_result(&mut conn);

    if let Ok(studio_name) = studio_name {
        return Json(
            AnimeStudio::belonging_to(&studio_name)
                .inner_join(schema::anime::table)
                .select(Anime::as_select())
                .load(&mut conn)
                .expect("Failed"),
        );
    }

    Json(Vec::new())
}
