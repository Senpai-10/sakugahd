use crate::models::show::Show;
use crate::schema::movies;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie {
    pub id: Uuid,
    pub show_id: Uuid,
    pub watch_after: i32,
    pub title: String,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Show))]
pub struct Movie {
    pub id: Uuid,
    pub show_id: Uuid,
    pub watch_after: i32,
    pub title: String,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}
