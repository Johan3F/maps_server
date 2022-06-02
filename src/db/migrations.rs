use rocket::{Build, Rocket};

use super::DbConn;

embed_migrations!("src/db/migrations/");

pub async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    println!("Migrating db. This could take some time. Please wait");
    let db = DbConn::get_one(&rocket)
        .await
        .expect("Unable to connect to database for executing migrations");
    db.run(
        |conn| match embedded_migrations::run_with_output(&*conn, &mut std::io::stdout()) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        },
    )
    .await
}
