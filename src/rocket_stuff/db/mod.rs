pub mod migrations;
pub mod schema;

use diesel::{query_dsl::RunQueryDsl, Connection, PgConnection};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel, Config};

#[database("maps")]
pub struct DbConn(diesel::PgConnection);

pub async fn create_db_if_not_exists(
    rocket: Rocket<Build>,
) -> Result<Rocket<Build>, Rocket<Build>> {
    let connection_string = match Config::from("maps", &rocket) {
        Ok(config) => config.url,
        Err(_) => panic!("Unable to retrieve DB's connection string"),
    };

    if PgConnection::establish(&connection_string).is_ok() {
        return Ok(rocket);
    }

    let (db_url, db_name) = connection_string.rsplit_once('/').unwrap();
    let conn = PgConnection::establish(db_url)
        .expect("Unable to connect to DB server for creating the DB");
    diesel::sql_query(&format!("CREATE DATABASE {};", db_name))
        .execute(&conn)
        .expect("Unable to run migrations");
    Ok(rocket)
}
