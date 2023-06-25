use async_trait::async_trait;
use std::sync::Arc;

use crate::db::Database;

#[async_trait]
pub trait Repo {}

pub struct DatabaseRepo {
    db_pool: Arc<Database>,
}

impl DatabaseRepo {
    pub fn new(db_pool: Arc<Database>) -> DatabaseRepo {
        DatabaseRepo { db_pool }
    }
}

#[async_trait]
impl Repo for DatabaseRepo {}
