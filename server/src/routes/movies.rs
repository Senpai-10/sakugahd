use crate::db::establish_connection;
use crate::models::movie::Movie;
use crate::schema::movies::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/movies/<id>")]
pub fn movies(id: Uuid) -> Json<Vec<Movie>> {
    let mut conn = establish_connection();

    Json(
        dsl::movies
            .filter(dsl::show_id.eq(id))
            .order(dsl::title)
            .load(&mut conn)
            .expect("Can't load movies"),
    )
}
