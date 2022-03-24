use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error},
};
use rocket::{
    delete, get,
    http::Status,
    post,
    serde::json::{serde_json::json, Json},
};

use crate::{
    api::models::ApiResponse,
    domain::models::{collection::Collection, DbConn},
    schema::collections,
};

#[post("/", data = "<new_collection>")]
pub async fn post_collection(conn: DbConn, new_collection: Json<Collection>) -> ApiResponse {
    let result = conn
        .run(|c| {
            diesel::insert_into(collections::table)
                .values(new_collection.into_inner())
                .get_result::<Collection>(c)
        })
        .await;
    match result {
        Ok(collection) => ApiResponse::new_ok(json!(collection)),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            ApiResponse::new_error("Collection already exists", Status::AlreadyReported)
        }
        Err(error) => ApiResponse::new_error(
            &format!("Unable to create new collection: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[get("/")]
pub async fn get_collections(conn: DbConn) -> ApiResponse {
    let result = conn.run(|c| collections::table.load::<Collection>(c)).await;
    match result {
        Ok(collections) => ApiResponse::new_ok(json!(collections)),
        Err(error) => ApiResponse::new_error(
            &format!("Unable to get collections: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[delete("/", data = "<collection_to_remove>")]
pub async fn delete_collection(
    conn: DbConn,
    collection_to_remove: Json<Collection>,
) -> ApiResponse {
    let collection_name = collection_to_remove.0.name.clone();
    let result = conn
        .run(|c| {
            diesel::delete(
                collections::table.filter(collections::name.eq_all(collection_to_remove.0.name)),
            )
            .execute(c)
        })
        .await;
    match result {
        Ok(_) => ApiResponse::new_ok(json!({ "removed": collection_name })),
        Err(error) => ApiResponse::new_error(
            &format!("Unable to remove collection {}: {}", collection_name, error),
            Status::InternalServerError,
        ),
    }
}
