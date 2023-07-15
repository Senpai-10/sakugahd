use crate::db::establish_connection;
use crate::models::episode::Episode;
use crate::schema::episodes::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/episodes/<id>")]
pub fn episodes(id: Uuid) -> Json<Vec<Episode>> {
    let mut conn = establish_connection();

    Json(
        dsl::episodes
            .filter(dsl::show_id.eq(id))
            .order(dsl::number)
            .load(&mut conn)
            .expect("Can't load episodes"),
    )
}
