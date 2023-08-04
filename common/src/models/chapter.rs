use crate::models::manga::Manga;
use crate::schema::chapters;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = chapters)]
pub struct NewChapter {
    pub id: String,
    pub manga_title: String,
    pub title: String,
    pub number: String,
    pub cover_image: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Manga, foreign_key = manga_title))]
pub struct Chapter {
    pub id: String,
    pub manga_title: String,
    pub title: String,
    pub number: String,
    pub cover_image: String,
}
