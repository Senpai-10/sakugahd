use crate::schema::studios;

#[derive(Insertable)]
#[diesel(table_name = studios)]
pub struct NewStudio {
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[primary_key(name)]
pub struct Studio {
    pub name: String,
}
