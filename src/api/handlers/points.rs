use axum::{routing::get, Json, Router};
use std::{sync::Arc, vec::Vec};

use super::error::Error;
use crate::{db::Database, domain::points};

type Result<T> = std::result::Result<T, Error>;
type DynRepo = Arc<dyn points::Repo + Send + Sync>;

pub fn add_routes(db_pool: Arc<Database>) -> Router {
    let repo = Arc::new(points::DatabaseRepo::new(db_pool)) as DynRepo;

    Router::new()
        .route("/", get(get_collections_points))
        .with_state(repo)
}

async fn get_collections_points() -> Result<Json<Vec<()>>> {
    Ok(Json(vec![()]))
}
