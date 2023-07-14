use crate::models::show::Show;
use crate::schema::episodes;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = episodes)]
pub struct NewEpisode {
    pub id: Uuid,
    pub show_id: Uuid,
    pub title: String,
    pub number: i32,
    pub is_filler: bool,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Show))]
pub struct Episode {
    pub id: Uuid,
    pub show_id: Uuid,
    pub title: String,
    pub number: i32,
    pub is_filler: bool,
    pub file_name: String,
    pub thumbnail: Option<Vec<u8>>,
}
