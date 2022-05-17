mod api;
mod domain;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::{launch, routes, Build, Rocket};

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

#[launch]
fn rocket() -> Rocket<Build> {
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
