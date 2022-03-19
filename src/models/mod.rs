use rocket_sync_db_pools::{database, diesel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Collection {
    pub name: String,
}

impl Collection {
    pub fn new(name: &str) -> Collection {
        Collection {
            name: name.to_owned(),
        }
    }
}

#[database("collections")]
pub struct CollectionsDbConn(diesel::SqliteConnection);
