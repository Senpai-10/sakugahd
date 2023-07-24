use crate::models::anime::Anime;
use crate::schema::openings;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = openings)]
pub struct NewOpening {
    pub id: String,
    pub anime_title: String,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail_file_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Anime, foreign_key = anime_title))]
pub struct Opening {
    pub id: String,
    pub anime_title: String,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail_file_name: String,
}
