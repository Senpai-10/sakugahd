use crate::models::show::Show;
use crate::models::studio::Studio;
use crate::schema::shows_studios;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = shows_studios)]
pub struct NewShowStudio {
    pub show_title: String,
    pub studio_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = shows_studios)]
#[diesel(belongs_to(Show, foreign_key = show_title))]
#[diesel(belongs_to(Studio, foreign_key = studio_name))]
pub struct ShowStudio {
    pub id: i32,
    pub show_title: String,
    pub studio_name: String,
}
