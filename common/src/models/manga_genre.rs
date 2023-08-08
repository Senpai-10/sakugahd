use crate::models::genre::Genre;
use crate::models::manga::Manga;
use crate::schema::manga_genres;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = manga_genres)]
pub struct NewMangaGenre {
    pub manga_title: String,
    pub genre_name: String,
}
#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = manga_genres)]
#[diesel(belongs_to(Manga, foreign_key = manga_title))]
#[diesel(belongs_to(Genre, foreign_key = genre_name))]
pub struct MangaGenre {
    pub id: i32,
    pub manga_title: String,
    pub genre_name: String,
}
