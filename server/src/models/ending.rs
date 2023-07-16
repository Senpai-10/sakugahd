use crate::models::show::Show;
use crate::schema::endings;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = endings)]
pub struct NewEnding {
    pub id: Uuid,
    pub show_title: String,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Show, foreign_key = show_title))]
pub struct Ending {
    pub id: Uuid,
    pub show_title: String,
    pub title: String,
    pub number: i32,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}
