use crate::schema::studios;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = studios)]
pub struct NewStudio {
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(primary_key(name))]
pub struct Studio {
    pub name: String,
}
