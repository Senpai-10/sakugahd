use crate::db::establish_connection;
use crate::models::opening::Opening;
use crate::schema::openings::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/openings/<id>")]
pub fn openings(id: Uuid) -> Json<Vec<Opening>> {
    let mut conn = establish_connection();

    Json(
        dsl::openings
            .filter(dsl::show_id.eq(id))
            .order(dsl::title)
            .load(&mut conn)
            .expect("Can't load openings"),
    )
}
