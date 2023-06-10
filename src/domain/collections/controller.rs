use deadpool::managed::Object;
use deadpool_diesel::Manager;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

use super::models::Collection;
use crate::{db::collections, db::collections as DbCollection, utils::to_anyhow};

#[derive(thiserror::Error, Debug)]
pub enum CollectionError {
    #[error("unknown error")]
    Unknown {
        #[from]
        source: anyhow::Error,
    },
    #[error("collection not found")]
    NotFound { id: String },
}

pub struct Controller {}

impl Controller {
    pub async fn get_collections(
        db_connection: Object<Manager<PgConnection>>,
    ) -> anyhow::Result<Vec<Collection>> {
        let collections = db_connection
            .interact(|connection: &mut PgConnection| {
                collections::table
                    .select(Collection::as_select())
                    .load(connection)
            })
            .await
            .map_err(to_anyhow)??;

        Ok(collections)
    }

    pub async fn get_collection(
        db_connection: Object<Manager<PgConnection>>,
        collection_id: Uuid,
    ) -> anyhow::Result<Collection> {
        let collection = db_connection
            .interact(move |connection: &mut PgConnection| {
                collections::table
                    .select(Collection::as_select())
                    .filter(DbCollection::id.eq(collection_id))
                    .first(connection)
                    .optional()
            })
            .await
            .map_err(to_anyhow)??;

        match collection {
            Some(collection) => Ok(collection),
            None => Err(anyhow::anyhow!(CollectionError::NotFound {
                id: format!("{collection_id}"),
            })),
        }
    }
}
