use crate::models::show::Show;
use crate::models::studio::Studio;
use crate::schema::shows_studios;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = shows_studios)]
pub struct NewShowStudio {
    pub show_id: Uuid,
    pub studio_name: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = shows_studios)]
#[diesel(belongs_to(Show))]
#[diesel(belongs_to(Studio, foreign_key = studio_name))]
pub struct ShowStudio {
    pub id: i32,
    pub show_id: Uuid,
    pub studio_name: String,
}
