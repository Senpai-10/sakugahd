use crate::db::establish_connection;
use crate::models::movie::Movie;
use crate::schema::movies::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/shows/<title>/movies")]
pub fn movies(title: String) -> Json<Vec<Movie>> {
    let mut conn = establish_connection();

    Json(
        dsl::movies
            .filter(dsl::show_title.eq(title))
            .order(dsl::title)
            .load(&mut conn)
            .expect("Can't load movies"),
    )
}
