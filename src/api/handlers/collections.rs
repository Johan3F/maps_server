use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use std::vec::Vec;
use uuid::Uuid;

use super::error::internal_error;
use crate::{
    db::Database,
    domain::collections::{Collection, Controller},
};

pub fn add_routes(db_pool: Database) -> Router {
    Router::new()
        .route("/", get(get_collections))
        .route("/:collection_id", get(get_collection))
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

async fn get_collection(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(collection_id): Path<Uuid>,
) -> Result<Json<Collection>, (StatusCode, String)> {
    let db_connection = pool.get().await.map_err(internal_error)?;

    let collection = Controller::get_collection(db_connection, collection_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(collection))
}
