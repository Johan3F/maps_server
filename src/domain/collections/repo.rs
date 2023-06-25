use anyhow::anyhow;
use async_trait::async_trait;
use diesel::{
    insert_into,
    prelude::*,
    result::{DatabaseErrorKind, Error as dieselError},
    PgConnection,
};
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
        let new_collection_name = new_collection.name.clone();

        let collection = db_connection
            .interact(move |connection: &mut PgConnection| {
                insert_into(collections::table)
                    .values(&new_collection)
                    .get_result::<Collection>(connection)
            })
            .await?;

        match collection {
            Ok(collection) => Ok(collection),
            Err(error) => match error {
                dieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Err(Error::AlreadyExists {
                        name: new_collection_name,
                    })
                }
                error => Err(Error::Unknown {
                    source: anyhow!(error),
                }),
            },
        }
    }
}
