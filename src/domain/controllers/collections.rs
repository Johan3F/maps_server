use anyhow::anyhow;
use uuid::Uuid;

use diesel::{prelude::*, result::DatabaseErrorKind};

use crate::{
    db::{schema::collections, DbConn},
    domain::models::collection::{Collection, CollectionNew},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown {
        #[from]
        source: anyhow::Error,
    },
    #[error("collection already exists")]
    AlreadyExists { name: String },
    #[error("collection not found")]
    NotFound { id: String },
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct CollectionsController {}

impl CollectionsController {
    pub async fn get_collections(conn: DbConn) -> Result<Vec<Collection>> {
        let result = conn.run(|c| collections::table.load::<Collection>(c)).await;
        match result {
            Ok(collections) => Ok(collections),
            Err(error) => Err(Error::Unknown {
                source: anyhow!(error),
            }),
        }
    }

    pub async fn create_collection(
        conn: DbConn,
        new_collection: CollectionNew,
    ) -> Result<Collection> {
        let name_to_create = new_collection.name.clone();

        match conn
            .run(|c| {
                diesel::insert_into(collections::table)
                    .values(new_collection)
                    .get_result::<Collection>(c)
            })
            .await
        {
            Ok(collection) => Ok(collection),
            Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Err(Error::AlreadyExists {
                    name: name_to_create,
                })
            }
            Err(error) => Err(Error::Unknown {
                source: anyhow!(error),
            }),
        }
    }

    pub async fn update_collection(
        conn: DbConn,
        collection_with_updates: Collection,
    ) -> Result<Collection> {
        let id_to_update = collection_with_updates.id.clone();
        match conn
            .run(move |c| {
                diesel::update(
                    collections::table.filter(collections::id.eq(collection_with_updates.id)),
                )
                .set(collections::name.eq(collection_with_updates.name))
                .get_result::<Collection>(c)
            })
            .await
        {
            Ok(updated_collection) => Ok(updated_collection),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound {
                id: format!("{id_to_update}"),
            }),
            Err(error) => Err(Error::Unknown {
                source: anyhow!(error),
            }),
        }
    }

    pub async fn delete_collection(conn: DbConn, collection_to_delete: Uuid) -> Result<()> {
        match conn
            .run(move |c| {
                diesel::delete(collections::table.filter(collections::id.eq(collection_to_delete)))
                    .execute(c)
            })
            .await
        {
            Ok(1) => Ok(()),
            Ok(0) => Err(Error::NotFound {
                id: format!("{collection_to_delete}"),
            }),
            Ok(removed_amount) => Err(Error::Unknown {
                source: anyhow!("unexpected error happen: removed '{removed_amount}' records when deleting collection '{collection_to_delete}'"),
            }),
            Err(error) => Err(Error::Unknown {
                source: anyhow!(error),
            }),
        }
    }
}
