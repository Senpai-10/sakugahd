use crate::models::show::Show;
use crate::schema::episodes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = episodes)]
pub struct NewEpisode {
    pub id: Uuid,
    pub show_title: String,
    pub title: String,
    pub number: i32,
    pub is_filler: bool,
    pub file_name: String,
    pub thumbnail: Vec<u8>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Show, foreign_key = show_title))]
pub struct Episode {
    pub id: Uuid,
    pub show_title: String,
    pub title: String,
    pub number: i32,
    pub is_filler: bool,
    pub file_name: String,
    pub thumbnail: Vec<u8>,
}
