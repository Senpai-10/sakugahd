use crate::models::genre::Genre;
use crate::models::show::Show;
use crate::schema::shows_genres;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = shows_genres)]
pub struct NewShowGenre {
    pub show_id: Uuid,
    pub genre_name: String,
}
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = shows_genres)]
#[diesel(belongs_to(Show))]
#[diesel(belongs_to(Genre, foreign_key = genre_name))]
pub struct ShowGenre {
    pub id: i32,
    pub show_id: Uuid,
    pub genre_name: String,
}
