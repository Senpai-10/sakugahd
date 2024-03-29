use crate::db::establish_connection;
use common::models::movie::Movie;
use common::schema::movies::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/anime/<title>/movies")]
pub fn movies(title: String) -> Json<Vec<Movie>> {
    let mut conn = establish_connection();

    Json(
        dsl::movies
            .filter(dsl::anime_title.eq(title))
            .order(dsl::number)
            .load(&mut conn)
            .expect("Can't load movies"),
    )
}
