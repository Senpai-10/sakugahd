use crate::models::show::Show;
use crate::schema::endings;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = endings)]
pub struct NewEnding {
    pub id: Uuid,
    pub show_id: Uuid,
    pub title: String,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Show))]
pub struct Ending {
    pub id: Uuid,
    pub show_id: Uuid,
    pub title: String,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}
