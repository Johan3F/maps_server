use axum::{routing::get, Json, Router};
use serde::Serialize;

use std::vec::Vec;

pub fn add_routes() -> Router {
    Router::new().route("/", get(get_collections))
}

async fn get_collections() -> Json<Vec<Collection>> {
    Json(vec![Collection {
        name: "First collection".to_owned(),
    }])
}

#[derive(Serialize)]
struct Collection {
    name: String,
}
