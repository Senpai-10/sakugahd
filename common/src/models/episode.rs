use crate::models::anime::Anime;
use crate::schema::episodes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = episodes)]
pub struct NewEpisode {
    pub id: String,
    pub anime_title: String,
    pub title: String,
    pub number: i32,
    pub is_filler: bool,
    pub file_name: String,
    pub thumbnail_file_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Anime, foreign_key = anime_title))]
pub struct Episode {
    pub id: String,
    pub anime_title: String,
    pub title: String,
    pub number: i32,
    pub is_filler: bool,
    pub file_name: String,
    pub thumbnail_file_name: String,
}
