use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error},
};
use rocket::{
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
        Err(error) => match error {
            Error::DatabaseError(kind, _) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    ApiResponse::new_error("Collection already exists", Status::AlreadyReported)
                }
                _ => ApiResponse::new_error(
                    &format!("Unable to create new collection: {:?}", error),
                    Status::InternalServerError,
                ),
            },
            _ => ApiResponse::new_error(
                &format!("Unable to create new collection: {}", error),
                Status::InternalServerError,
            ),
        },
    }
}
