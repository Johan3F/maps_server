use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::db::collections;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = collections)]
pub struct Collection {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = collections)]
pub struct CollectionNoID {
    pub name: String,
}
