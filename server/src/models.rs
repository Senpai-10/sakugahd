use crate::schema::shows;
use uuid::Uuid;

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug)]
#[ExistingTypePath = "crate::schema::sql_types::ShowFormat"]
#[DbValueStyle = "UPPERCASE"]
pub enum ShowFormat {
    TV,
    OVA,
    ONA,
    MOVIE,
    SPECIAL,
}

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug)]
#[ExistingTypePath = "crate::schema::sql_types::ShowStatus"]
#[DbValueStyle = "UPPERCASE"]
pub enum ShowStatus {
    FINISHED,
    ONGOING,
}

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug)]
#[ExistingTypePath = "crate::schema::sql_types::ShowSeason"]
#[DbValueStyle = "UPPERCASE"]
pub enum ShowSeason {
    SPRING,
    SUMMER,
    FALL,
    WINTER,
}

#[derive(Insertable)]
#[diesel(table_name = shows)]
pub struct NewShow<'a> {
    pub id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub format: Option<ShowFormat>,
    pub status: Option<ShowStatus>,
    pub season: Option<ShowSeason>,
    pub season_year: Option<i32>,
    pub folder_name: &'a str,
    pub image: Vec<u8>,
    pub banner: Vec<u8>,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Show {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub format: Option<ShowFormat>,
    pub status: Option<ShowStatus>,
    pub season: Option<ShowSeason>,
    pub season_year: Option<i32>,
    pub folder_name: String,
    pub image: Vec<u8>,
    pub banner: Vec<u8>,
}
