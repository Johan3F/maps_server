mod api;
mod domain;
mod schema;

#[macro_use]
extern crate diesel;

use rocket::*;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(domain::models::DbConn::fairing())
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
