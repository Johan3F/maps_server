use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::WithRejection;
use std::{sync::Arc, vec::Vec};
use uuid::Uuid;

use super::error::Error;
use crate::{
    db::Database,
    domain::collections::{self, Collection, NewCollection},
};

type Result<T> = std::result::Result<T, Error>;
type DynRepo = Arc<dyn collections::Repo + Send + Sync>;

pub fn add_routes(db_pool: Arc<Database>) -> Router {
    let repo = Arc::new(collections::DatabaseRepo::new(db_pool)) as DynRepo;

    Router::new()
        .route("/", get(get_collections))
        .route("/:collection_id", get(get_collection))
        .route("/", post(create_collection))
        .route("/", put(update_collection))
        .route("/:collection_id", delete(delete_collection))
        .with_state(repo)
}

async fn get_collections(State(repo): State<DynRepo>) -> Result<Json<Vec<Collection>>> {
    let collections = repo.get_collections().await?;
    Ok(Json(collections))
}

async fn get_collection(
    State(repo): State<DynRepo>,
    WithRejection(Path(collection_id), _): WithRejection<Path<Uuid>, Error>,
) -> Result<Json<Collection>> {
    let collection = repo.get_collection(collection_id).await?;
    Ok(Json(collection))
}

async fn create_collection(
    State(repo): State<DynRepo>,
    WithRejection(Json(new_collection), _): WithRejection<Json<NewCollection>, Error>,
) -> Result<Json<Collection>> {
    let collection = repo.create_collection(new_collection).await?;
    Ok(Json(collection))
}

async fn update_collection(
    State(repo): State<DynRepo>,
    WithRejection(Json(collection), _): WithRejection<Json<Collection>, Error>,
) -> Result<Json<Collection>> {
    let collection = repo.update_collection(collection).await?;
    Ok(Json(collection))
}

async fn delete_collection(
    State(repo): State<DynRepo>,
    WithRejection(Path(collection_id), _): WithRejection<Path<Uuid>, Error>,
) -> Result<Json<Collection>> {
    let collection = repo.delete_collection(collection_id).await?;
    Ok(Json(collection))
}
