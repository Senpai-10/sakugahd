use crate::models::manga::Manga;
use crate::models::theme::Theme;
use crate::schema::manga_themes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = manga_themes)]
pub struct NewMangaTheme {
    pub manga_title: String,
    pub theme_name: String,
}
#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = manga_themes)]
#[diesel(belongs_to(Manga, foreign_key = manga_title))]
#[diesel(belongs_to(Theme, foreign_key = theme_name))]
pub struct MangaTheme {
    pub id: i32,
    pub manga_title: String,
    pub theme_name: String,
}
