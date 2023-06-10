mod schema;
pub use schema::*;

mod migrations;
pub use migrations::*;

use deadpool_diesel::{Manager, Pool, Runtime::Tokio1};
use diesel::PgConnection;

pub async fn get_db_pool(db_url: &str) -> anyhow::Result<Pool<Manager<PgConnection>>> {
    // set up connection pool
    let manager = Manager::new(db_url, Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager).build()?;

    run_migrations(&pool).await?;

    Ok(pool)
}
