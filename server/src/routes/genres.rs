use crate::db::establish_connection;
use common::models::anime::Anime;
use common::models::anime_genre::AnimeGenre;
use common::models::genre::Genre;
use common::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/genres")]
pub fn genres() -> Json<Vec<String>> {
    let mut conn = establish_connection();

    Json(
        schema::genres::dsl::genres
            .select(schema::genres::name)
            .order(schema::genres::dsl::name)
            .load(&mut conn)
            .expect("Can't load genres"),
    )
}

#[get("/genres/<name>")]
pub fn genre(name: String) -> Json<Vec<Anime>> {
    let mut conn = establish_connection();

    let genre_name = schema::genres::table
        .filter(schema::genres::dsl::name.eq(&name))
        .select(Genre::as_select())
        .get_result(&mut conn);

    if let Ok(genre_name) = genre_name {
        return Json(
            AnimeGenre::belonging_to(&genre_name)
                .inner_join(schema::anime::table)
                .select(Anime::as_select())
                .load(&mut conn)
                .expect("Failed"),
        );
    }

    Json(Vec::new())
}
