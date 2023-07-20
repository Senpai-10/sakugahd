use crate::db::establish_connection;
use common::models::episode::Episode;
use common::schema::episodes::dsl;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/shows/<title>/episodes")]
pub fn episodes(title: String) -> Json<Vec<Episode>> {
    let mut conn = establish_connection();

    Json(
        dsl::episodes
            .filter(dsl::show_title.eq(title))
            .order(dsl::number)
            .load(&mut conn)
            .expect("Can't load episodes"),
    )
}
