use crate::models::show::Show;
use crate::schema::openings;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = openings)]
pub struct NewOpening {
    pub id: Uuid,
    pub show_id: Uuid,
    pub title: String,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(belongs_to(Show))]
pub struct Opening {
    pub id: Uuid,
    pub show_id: Uuid,
    pub title: String,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}
