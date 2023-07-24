use crate::db::establish_connection;
use common::models::ending::Ending;
use common::schema::endings::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/anime/<title>/endings")]
pub fn endings(title: String) -> Json<Vec<Ending>> {
    let mut conn = establish_connection();

    Json(
        dsl::endings
            .filter(dsl::anime_title.eq(title))
            .order(dsl::number)
            .load(&mut conn)
            .expect("Can't load openings"),
    )
}
