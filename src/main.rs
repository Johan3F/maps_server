mod api;
mod domain;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::{Connection, PgConnection};
use rocket::fairing::AdHoc;
use rocket::{launch, routes, Build, Rocket};
use std::env;

embed_migrations!("migrations/");

async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    println!("Migrating db. This could take some time. Please wait");
    let db = domain::models::DbConn::get_one(&rocket)
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

// TODO: Trying to create DB on startup Check this: https://github.com/diesel-rs/diesel/discussions/3129
fn create_db_if_not_exists<Conn: Connection>() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("ROCKET_DATABASES_MAPS")?;

    println!("db_url: {}", db_url);

    if Conn::establish(&db_url).is_ok() {
        return Ok(())
    }

    let (db_url, db_name) = db_url.rsplit_once('/').unwrap();
    let conn = Conn::establish(db_url)?;
    conn.execute(&("CREATE DATABASE ".to_owned() + db_name + ";"))?;
    Ok(())
}

#[launch]
fn rocket() -> Rocket<Build> {
    create_db_if_not_exists::<PgConnection>().expect("Unable to create DB");

    rocket::build()
        .attach(domain::models::DbConn::fairing())
        .attach(AdHoc::try_on_ignite("Database migrator", migrate))
        .mount("/", routes![api::handlers::support::health])
        .mount(
            "/collections",
            routes![
                api::handlers::collections::post_collection,
                api::handlers::collections::get_collections,
                api::handlers::collections::delete_collection,
                api::handlers::collections::update_collection,
            ],
        )
}
