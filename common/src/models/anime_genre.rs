use crate::models::anime::Anime;
use crate::models::genre::Genre;
use crate::schema::anime_genres;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = anime_genres)]
pub struct NewAnimeGenre {
    pub anime_title: String,
    pub genre_name: String,
}
#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = anime_genres)]
#[diesel(belongs_to(Anime, foreign_key = anime_title))]
#[diesel(belongs_to(Genre, foreign_key = genre_name))]
pub struct AnimeGenre {
    pub id: i32,
    pub anime_title: String,
    pub genre_name: String,
}
