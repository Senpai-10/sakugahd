use crate::models::show::Show;
use crate::schema::openings;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = openings)]
pub struct NewOpening {
    pub id: Uuid,
    pub show_title: String,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail: Vec<u8>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Show, foreign_key = show_title))]
pub struct Opening {
    pub id: Uuid,
    pub show_title: String,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail: Vec<u8>,
}
