use diesel::prelude::*;
use models::{NewShow, Show, ShowFormat, ShowSeason, ShowStatus};
use schema::shows::dsl::*;
use uuid::Uuid;

pub fn load_shows() {
    let anime_folder = env::var("ANIME_FOLDER").expect("ANIME_FOLDER must be set");

    // let new_show = NewShow {
    //     id: Uuid::new_v4(),
    //     title: "bleach",
    //     description: "some description.",
    //     format: None,
    //     status: None,
    //     season: None,
    //     season_year: None,
    //     folder_name: "Bleach (English SUB)",
    //     image: vec![0],
    //     banner: vec![0],
    // };

    // diesel::insert_into(shows)
    //     .values(&new_show)
    //     .execute(&mut connection)
    //     .expect("Error saving new show");
}
