use async_trait::async_trait;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

use super::{error::Error, models::Collection};
use crate::db::{collections, Database};

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Repo {
    async fn get_collections(&self) -> Result<Vec<Collection>>;
    async fn get_collection(&self, collection_id: Uuid) -> Result<Collection>;
}

pub struct DatabaseRepo {
    db_pool: Database,
}

impl DatabaseRepo {
    pub fn new(db_pool: Database) -> DatabaseRepo {
        DatabaseRepo { db_pool }
    }
}

#[async_trait]
impl Repo for DatabaseRepo {
    async fn get_collections(&self) -> Result<Vec<Collection>> {
        let db_connection = self.db_pool.get().await?;

        let collections = db_connection
            .interact(|connection: &mut PgConnection| {
                collections::table
                    .select(Collection::as_select())
                    .load(connection)
            })
            .await??;

        Ok(collections)
    }

    async fn get_collection(&self, collection_id: Uuid) -> Result<Collection> {
        let db_connection = self.db_pool.get().await?;

        let collection = db_connection
            .interact(move |connection: &mut PgConnection| {
                collections::table
                    .select(Collection::as_select())
                    .filter(collections::id.eq(collection_id))
                    .first(connection)
                    .optional()
            })
            .await??;

        match collection {
            Some(collection) => Ok(collection),
            None => Err(Error::NotFound { id: collection_id }),
        }
    }
}
