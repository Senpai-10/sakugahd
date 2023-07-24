use crate::models::anime::Anime;
use crate::models::studio::Studio;
use crate::schema::anime_studios;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = anime_studios)]
pub struct NewAnimeStudio {
    pub anime_title: String,
    pub studio_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = anime_studios)]
#[diesel(belongs_to(Anime, foreign_key = anime_title))]
#[diesel(belongs_to(Studio, foreign_key = studio_name))]
pub struct AnimeStudio {
    pub id: i32,
    pub anime_title: String,
    pub studio_name: String,
}
