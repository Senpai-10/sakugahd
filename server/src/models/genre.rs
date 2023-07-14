use crate::schema::genres;

#[derive(Insertable)]
#[diesel(table_name = genres)]
pub struct NewGenre {
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(primary_key(name))]
pub struct Genre {
    pub name: String,
}
