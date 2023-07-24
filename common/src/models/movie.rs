use crate::models::anime::Anime;
use crate::schema::movies;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = movies)]
pub struct NewMovie {
    pub id: String,
    pub anime_title: String,
    pub watch_after: i32,
    pub number: i32,
    pub title: String,
    pub file_name: String,
    pub thumbnail_file_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Anime, foreign_key = anime_title))]
pub struct Movie {
    pub id: String,
    pub anime_title: String,
    pub watch_after: i32,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail_file_name: String,
}
