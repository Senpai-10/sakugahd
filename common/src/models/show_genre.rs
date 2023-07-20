use crate::models::genre::Genre;
use crate::models::show::Show;
use crate::schema::shows_genres;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = shows_genres)]
pub struct NewShowGenre {
    pub show_title: String,
    pub genre_name: String,
}
#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = shows_genres)]
#[diesel(belongs_to(Show, foreign_key = show_title))]
#[diesel(belongs_to(Genre, foreign_key = genre_name))]
pub struct ShowGenre {
    pub id: i32,
    pub show_title: String,
    pub genre_name: String,
}
