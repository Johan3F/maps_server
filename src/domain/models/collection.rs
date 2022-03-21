use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::collections;

#[derive(Serialize, Queryable, Deserialize, Insertable)]
#[table_name = "collections"]
pub struct Collection {
    pub name: String,
}
