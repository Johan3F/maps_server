use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::db::collections;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Selectable)]
pub struct Collection {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = collections)]
pub struct NewCollection {
    pub name: String,
}
