use crate::db::establish_connection;
use common::models::show::Show;
use common::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/shows")]
pub fn shows() -> Json<Vec<Show>> {
    let mut conn = establish_connection();

    Json(
        schema::shows::dsl::shows
            .load(&mut conn)
            .expect("Can't load shows"),
    )
}
