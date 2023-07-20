use crate::db::establish_connection;
use common::models::opening::Opening;
use common::schema::openings::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/shows/<title>/openings")]
pub fn openings(title: String) -> Json<Vec<Opening>> {
    let mut conn = establish_connection();

    Json(
        dsl::openings
            .filter(dsl::show_title.eq(title))
            .order(dsl::title)
            .load(&mut conn)
            .expect("Can't load openings"),
    )
}
