use crate::schema::anime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::AnimeFormat"]
#[DbValueStyle = "UPPERCASE"]
#[allow(clippy::upper_case_acronyms)]
pub enum AnimeFormat {
    TV,
    OVA,
    ONA,
    MOVIE,
    SPECIAL,
}

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::AnimeStatus"]
#[DbValueStyle = "UPPERCASE"]
#[allow(clippy::upper_case_acronyms)]
pub enum AnimeStatus {
    FINISHED,
    ONGOING,
}

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::AnimeSeason"]
#[DbValueStyle = "UPPERCASE"]
#[allow(clippy::upper_case_acronyms)]
pub enum AnimeSeason {
    SPRING,
    SUMMER,
    FALL,
    WINTER,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = anime)]
pub struct NewAnime {
    pub title: String,
    pub description: String,
    pub format: Option<AnimeFormat>,
    pub status: Option<AnimeStatus>,
    pub season: Option<AnimeSeason>,
    pub season_year: Option<i32>,
    pub cover: String,
}

#[derive(
    Debug, Queryable, AsChangeset, Identifiable, Selectable, PartialEq, Serialize, Deserialize,
)]
#[diesel(primary_key(title))]
#[table_name = "anime"]
pub struct Anime {
    pub title: String,
    pub description: String,
    pub format: Option<AnimeFormat>,
    pub status: Option<AnimeStatus>,
    pub season: Option<AnimeSeason>,
    pub season_year: Option<i32>,
    pub cover: String,
}
