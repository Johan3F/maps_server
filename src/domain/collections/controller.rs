use deadpool::managed::Object;
use deadpool_diesel::Manager;
use diesel::{prelude::*, PgConnection};

use super::models::Collection;
use crate::db::collections;

pub struct Controller {}

impl Controller {
    pub async fn get_collections(
        db_connection: Object<Manager<PgConnection>>,
    ) -> anyhow::Result<Vec<Collection>> {
        let res = db_connection
            .interact(|conn: &mut PgConnection| {
                collections::table
                    .select(Collection::as_select())
                    .load(conn)
            })
            .await
            .map_err(to_anyhow)??;

        Ok(res)
    }
}

fn to_anyhow<T>(error: T) -> anyhow::Error
where
    T: ToString,
{
    anyhow::anyhow!(error.to_string())
}
