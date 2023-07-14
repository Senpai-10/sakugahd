use crate::schema::shows;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ShowFormat"]
#[DbValueStyle = "UPPERCASE"]
pub enum ShowFormat {
    TV,
    OVA,
    ONA,
    MOVIE,
    SPECIAL,
}

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ShowStatus"]
#[DbValueStyle = "UPPERCASE"]
pub enum ShowStatus {
    FINISHED,
    ONGOING,
}

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ShowSeason"]
#[DbValueStyle = "UPPERCASE"]
pub enum ShowSeason {
    SPRING,
    SUMMER,
    FALL,
    WINTER,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = shows)]
pub struct NewShow {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub format: Option<ShowFormat>,
    pub status: Option<ShowStatus>,
    pub season: Option<ShowSeason>,
    pub season_year: Option<i32>,
    pub directory_name: String,
    pub image: Option<Vec<u8>>,
    pub banner: Option<Vec<u8>>,
}

#[derive(
    Debug, Queryable, AsChangeset, Identifiable, Selectable, PartialEq, Serialize, Deserialize,
)]
pub struct Show {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub format: Option<ShowFormat>,
    pub status: Option<ShowStatus>,
    pub season: Option<ShowSeason>,
    pub season_year: Option<i32>,
    pub directory_name: String,
    pub image: Option<Vec<u8>>,
    pub banner: Option<Vec<u8>>,
}
