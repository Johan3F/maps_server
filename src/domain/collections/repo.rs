use anyhow::anyhow;
use async_trait::async_trait;
use diesel::{delete, insert_into, prelude::*, update, PgConnection};
use std::sync::Arc;
use uuid::Uuid;

use super::{
    error::Error,
    models::{Collection, NewCollection},
};
use crate::db::{collections, Database};

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Repo {
    async fn get_collections(&self) -> Result<Vec<Collection>>;
    async fn get_collection(&self, collection_id: Uuid) -> Result<Collection>;
    async fn create_collection(&self, new_collection: NewCollection) -> Result<Collection>;
    async fn update_collection(&self, new_collection: Collection) -> Result<Collection>;
    async fn delete_collection(&self, collection_id: Uuid) -> Result<Collection>;
}

pub struct DatabaseRepo {
    db_pool: Arc<Database>,
}

impl DatabaseRepo {
    pub fn new(db_pool: Arc<Database>) -> DatabaseRepo {
        DatabaseRepo { db_pool }
    }
}

#[async_trait]
impl Repo for DatabaseRepo {
    async fn get_collections(&self) -> Result<Vec<Collection>> {
        let db_connection = self.db_pool.get().await?;

        let collections_list = db_connection
            .interact(|connection: &mut PgConnection| {
                collections::table
                    .select(Collection::as_select())
                    .load(connection)
            })
            .await??;

        Ok(collections_list)
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

    async fn create_collection(&self, new_collection: NewCollection) -> Result<Collection> {
        let db_connection = self.db_pool.get().await?;

        let collection = db_connection
            .interact(move |connection: &mut PgConnection| {
                insert_into(collections::table)
                    .values(&new_collection)
                    .get_result::<Collection>(connection)
            })
            .await??;

        Ok(collection)
    }

    async fn update_collection(&self, new_collection_value: Collection) -> Result<Collection> {
        let db_connection = self.db_pool.get().await?;

        let update_result = db_connection
            .interact(move |connection: &mut PgConnection| {
                update(collections::table)
                    .filter(collections::id.eq(new_collection_value.id))
                    .set(collections::name.eq(new_collection_value.name))
                    .get_result::<Collection>(connection)
            })
            .await?;

        match update_result {
            Ok(collection) => Ok(collection),
            Err(error) => match error {
                diesel::NotFound => Err(Error::NotFound {
                    id: new_collection_value.id,
                }),
                error => Err(Error::Unknown {
                    source: anyhow!(error),
                }),
            },
        }
    }

    async fn delete_collection(&self, collection_id: Uuid) -> Result<Collection> {
        let db_connection = self.db_pool.get().await?;

        let delete_result = db_connection
            .interact(move |connection: &mut PgConnection| {
                delete(collections::table)
                    .filter(collections::id.eq(collection_id))
                    .get_result::<Collection>(connection)
            })
            .await?;
        match delete_result {
            Ok(collection) => Ok(collection),
            Err(error) => match error {
                diesel::NotFound => Err(Error::NotFound { id: collection_id }),
                error => Err(Error::Unknown {
                    source: anyhow!(error),
                }),
            },
        }
    }
}
