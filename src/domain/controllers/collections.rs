use anyhow::anyhow;

use diesel::{prelude::*, result::DatabaseErrorKind};
use rocket::serde::uuid::Uuid;

use crate::{
    db::{schema::collections, DbConn},
    domain::models::collection::{Collection, CollectionNoID},
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

    pub async fn get_collection(conn: DbConn, collection_id: Uuid) -> Result<Collection> {
        let result = conn
            .run(move |c| {
                collections::table
                    .filter(collections::id.eq(&collection_id))
                    .first(c)
            })
            .await;
        match result {
            Ok(collection) => Ok(collection),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound {
                id: format!("{collection_id}"),
            }),
            Err(error) => Err(Error::Unknown {
                source: anyhow!(error),
            }),
        }
    }

    pub async fn create_collection(
        conn: DbConn,
        new_collection: CollectionNoID,
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
        collection_id: Uuid,
        modified_collection: CollectionNoID,
    ) -> Result<Collection> {
        match conn
            .run(move |c| {
                diesel::update(collections::table.filter(collections::id.eq(&collection_id)))
                    .set(collections::name.eq(modified_collection.name))
                    .get_result::<Collection>(c)
            })
            .await
        {
            Ok(updated_collection) => Ok(updated_collection),
            Err(diesel::result::Error::NotFound) => Err(Error::NotFound {
                id: format!("{collection_id}"),
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
