use serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable};
use rocket::serde::uuid::Uuid;

use crate::db::schema::collections;

#[derive(Serialize, Queryable, Deserialize, Insertable)]
#[table_name = "collections"]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "collections"]
pub struct CollectionNoID {
    pub name: String,
}
