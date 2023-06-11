use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use std::{sync::Arc, vec::Vec};
use uuid::Uuid;

use super::error::Error;
use crate::{
    db::Database,
    domain::collections::{self, Collection},
};

type Result<T> = std::result::Result<T, Error>;
type DynRepo = Arc<dyn collections::Repo + Send + Sync>;

pub fn add_routes(db_pool: Database) -> Router {
    let repo = Arc::new(collections::DatabaseRepo::new(db_pool)) as DynRepo;

    Router::new()
        .route("/", get(get_collections))
        .route("/:collection_id", get(get_collection))
        .with_state(repo)
}

async fn get_collections(State(repo): State<DynRepo>) -> Result<Json<Vec<Collection>>> {
    let collections = repo.get_collections().await?;
    Ok(Json(collections))
}

async fn get_collection(
    State(repo): State<DynRepo>,
    Path(collection_id): Path<Uuid>, // TODO: Make sure that the error when not passing a uuid is more user freindly
) -> Result<Json<Collection>> {
    let collection = repo.get_collection(collection_id).await?;
    Ok(Json(collection))
}
