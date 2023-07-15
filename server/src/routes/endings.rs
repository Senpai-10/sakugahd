use crate::db::establish_connection;
use crate::models::ending::Ending;
use crate::schema::endings::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/endings/<id>")]
pub fn endings(id: Uuid) -> Json<Vec<Ending>> {
    let mut conn = establish_connection();

    Json(
        dsl::endings
            .filter(dsl::show_id.eq(id))
            .order(dsl::title)
            .load(&mut conn)
            .expect("Can't load openings"),
    )
}
