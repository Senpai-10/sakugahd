use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, std::fmt::Debug, PartialEq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::TagTypes"]
#[DbValueStyle = "UPPERCASE"]
#[allow(clippy::upper_case_acronyms)]
pub enum TagTypes {
    ANIME,
    MANGA,
}
