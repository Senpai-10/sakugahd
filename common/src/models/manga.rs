use crate::schema::manga;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = manga)]
pub struct NewManga {
    pub title: String,
    pub description: String,
    pub cover: Option<String>,
}

#[derive(
    Debug, Queryable, AsChangeset, Identifiable, Selectable, PartialEq, Serialize, Deserialize,
)]
#[diesel(primary_key(title))]
#[diesel(table_name = manga)]
pub struct Manga {
    pub title: String,
    pub description: String,
    pub cover: Option<String>,
}
