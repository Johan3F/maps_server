mod handlers;
mod models;

use rocket::*;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(models::CollectionsDbConn::fairing())
        .mount("/", routes![handlers::support::health])
        .mount(
            "/collections",
            routes![
                handlers::collections::get_collection_points,
                handlers::collections::post_collection
            ],
        )
}
