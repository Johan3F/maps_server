mod api;
mod db;
mod domain;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::{launch, routes, Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(AdHoc::try_on_ignite(
            "Database creation",
            db::create_db_if_not_exists,
        ))
        .attach(db::DbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database migrator",
            db::migrations::migrate,
        ))
        .mount("/", routes![api::handlers::support::health])
        .mount(
            "/collections",
            routes![
                api::handlers::collections::get_collection,
                api::handlers::collections::post_collection,
                api::handlers::collections::get_collections,
                api::handlers::collections::update_collection,
                api::handlers::collections::delete_collection,
            ],
        )
}
