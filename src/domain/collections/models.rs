use serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable, Selectable};

use crate::db::collections;

#[derive(Serialize, Queryable, Deserialize, Selectable)]
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
