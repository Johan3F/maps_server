use anyhow::anyhow;
use thiserror::Error;

use diesel::{prelude::*, result::DatabaseErrorKind};

use crate::{
    db::{schema::collections, DbConn},
    domain::models::collection::{Collection, CollectionNew},
};

#[derive(Error, Debug)]
pub enum CollectionsError {
    #[error("unknown error")]
    Unknown {
        #[from]
        source: anyhow::Error,
    },

    #[error("collection already exists")]
    AlreadyExists { existing: String },
}

pub type Result<T> = core::result::Result<T, CollectionsError>;

pub struct CollectionsController {}

impl CollectionsController {
    pub async fn create_collection(
        conn: DbConn,
        new_collection: CollectionNew,
    ) -> Result<Collection> {
        let new_collection_name = new_collection.name.clone();

        let result = conn
            .run(|c| {
                diesel::insert_into(collections::table)
                    .values(new_collection)
                    .get_result::<Collection>(c)
            })
            .await;

        match result {
            Ok(collection) => Ok(collection),
            Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Err(CollectionsError::AlreadyExists {
                    existing: new_collection_name,
                })
            }
            Err(error) => Err(CollectionsError::Unknown {
                source: anyhow!(error),
            }),
        }
    }

    pub async fn get_collections(conn: DbConn) -> Result<Vec<Collection>> {
        let result = conn.run(|c| collections::table.load::<Collection>(c)).await;
        match result {
            Ok(collections) => Ok(collections),
            Err(error) => Err(CollectionsError::Unknown {
                source: anyhow!(error),
            }),
        }
    }
}
