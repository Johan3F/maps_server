use async_trait::async_trait;

use crate::db::Database;

#[async_trait]
pub trait Repo {}

pub struct DatabaseRepo {
    db_pool: Database,
}

impl DatabaseRepo {
    pub fn new(db_pool: Database) -> DatabaseRepo {
        DatabaseRepo { db_pool }
    }
}

#[async_trait]
impl Repo for DatabaseRepo {}
