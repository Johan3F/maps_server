use diesel::{Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::db::elements;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Selectable)]
#[diesel(table_name = elements)]
pub struct Point {
    pub id: uuid::Uuid,
    pub collection_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
}
