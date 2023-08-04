use crate::models::chapter::Chapter;
use crate::models::manga::Manga;
use crate::schema::pages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = pages)]
pub struct NewPage {
    pub id: String,
    pub manga_title: String,
    pub chapter_id: String,
    pub number: i32,
    pub file_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Manga, foreign_key = manga_title))]
#[diesel(belongs_to(Chapter, foreign_key = chapter_id))]
pub struct Page {
    pub id: String,
    pub manga_title: String,
    pub chapter_id: String,
    pub number: i32,
    pub file_name: String,
}
