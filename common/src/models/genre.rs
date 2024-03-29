use crate::enums::TagTypes;
use crate::schema::genres;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = genres)]
pub struct NewGenre {
    pub name: String,
    pub tag_type: TagTypes,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(primary_key(name))]
pub struct Genre {
    pub name: String,
    pub tag_type: TagTypes,
}
