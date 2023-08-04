use crate::enums::TagTypes;
use crate::schema::themes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = themes)]
pub struct NewTheme {
    pub name: String,
    pub tag_type: TagTypes,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(primary_key(name))]
pub struct Theme {
    pub name: String,
    pub tag_type: TagTypes,
}
