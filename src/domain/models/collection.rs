use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::collections;

#[derive(Serialize, Queryable, Deserialize, Insertable)]
#[table_name = "collections"]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Queryable, Deserialize, Insertable)]
#[table_name = "collections"]
pub struct CollectionNew {
    pub name: String,
}
