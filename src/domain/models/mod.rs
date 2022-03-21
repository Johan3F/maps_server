pub mod collection;

use rocket_sync_db_pools::{database, diesel};

#[database("maps")]
pub struct DbConn(diesel::PgConnection);
