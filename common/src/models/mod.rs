#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::TagTypes"]
#[DbValueStyle = "UPPERCASE"]
#[allow(clippy::upper_case_acronyms)]
pub enum TagTypes {
    ANIME,
    MANGA,
}

pub mod anime;
pub mod anime_genre;
pub mod anime_studio;
pub mod chapter;
pub mod ending;
pub mod episode;
pub mod genre;
pub mod manga;
pub mod manga_genre;
pub mod manga_theme;
pub mod movie;
pub mod opening;
pub mod page;
pub mod studio;
pub mod theme;
