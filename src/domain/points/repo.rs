use async_trait::async_trait;
use diesel::{prelude::*, PgConnection};
use std::sync::Arc;
use uuid::Uuid;

use super::{Error, Point};
use crate::db::{elements, Database};

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Repo {
    async fn get_collection_points(&self, collection: Uuid) -> Result<Vec<Point>>;
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
    async fn get_collection_points(&self, collection: Uuid) -> Result<Vec<Point>> {
        let db_connection = self.db_pool.get().await?;

        let collection_points = db_connection
            .interact(move |connection: &mut PgConnection| {
                elements::table
                    .select(Point::as_select())
                    .filter(elements::collection_id.eq(collection))
                    .load(connection)
            })
            .await??;

        Ok(collection_points)
    }
}
