use crate::db::Database;
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use std::vec::Vec;

pub fn add_routes(db_pool: Database) -> Router {
    Router::new()
        .route("/", get(get_collections))
        .with_state(db_pool)
}

async fn get_collections(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Json<Vec<Collection>> {
    Json(vec![Collection {
        name: "First collection".to_owned(),
    }])
}

#[derive(Serialize)]
struct Collection {
    name: String,
}
