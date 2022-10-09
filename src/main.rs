mod api;
mod db;
mod domain;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::{launch, Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    let building_rocket = rocket::build()
        .attach(AdHoc::try_on_ignite(
            "Database creation",
            db::create_db_if_not_exists,
        ))
        .attach(db::DbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database migrator",
            db::migrations::migrate,
        ));

    let building_rocket = api::handlers::support::add_routes(building_rocket);
    let building_rocket = api::handlers::collections::add_routes(building_rocket);
    building_rocket
}
