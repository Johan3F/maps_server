use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use std::vec::Vec;

use super::error::internal_error;
use crate::{
    db::Database,
    domain::collections::{Collection, Controller},
};

pub fn add_routes(db_pool: Database) -> Router {
    Router::new()
        .route("/", get(get_collections))
        .with_state(db_pool)
}

async fn get_collections(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Collection>>, (StatusCode, String)> {
    let db_connection = pool.get().await.map_err(internal_error)?;

    let collections = Controller::get_collections(db_connection)
        .await
        .map_err(internal_error)?;
    Ok(Json(collections))
}
