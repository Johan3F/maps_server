use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error},
};
use rocket::{
    delete, get,
    http::Status,
    patch, post,
    serde::json::{serde_json::json, Json},
};
use uuid::Uuid;

use crate::{
    api::models::ApiResponse,
    domain::models::{
        collection::{Collection, CollectionNew},
        DbConn,
    },
    schema::collections,
};

#[post("/", data = "<new_collection>")]
pub async fn post_collection(conn: DbConn, new_collection: Json<CollectionNew>) -> ApiResponse {
    let result = conn
        .run(|c| {
            diesel::insert_into(collections::table)
                .values(new_collection.into_inner())
                .get_result::<Collection>(c)
        })
        .await;
    match result {
        Ok(collection) => ApiResponse::new(json!(collection), Status::Created),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            ApiResponse::new_message("Collection already exists", Status::AlreadyReported)
        }
        Err(error) => ApiResponse::new_message(
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
        Err(error) => ApiResponse::new_message(
            &format!("Unable to get collections: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[delete("/", data = "<collection_to_remove>")]
pub async fn delete_collection(conn: DbConn, collection_to_remove: Json<Uuid>) -> ApiResponse {
    let collection_to_remove = collection_to_remove.0;

    let result = conn
        .run(move |c| {
            diesel::delete(collections::table.filter(collections::id.eq(collection_to_remove)))
                .execute(c)
        })
        .await;
    match result {
        Ok(1) => ApiResponse::new_message("removed", Status::Accepted),
        Ok(0) => ApiResponse::new_message("Collection not found", Status::NotFound),
        Ok(removed_collections) => ApiResponse::new_message(
            &format!(
                "Something went wrong. Removed {} collections!",
                removed_collections
            ),
            Status::InternalServerError,
        ),
        Err(error) => ApiResponse::new_message(
            &format!("Unable to remove collection: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[patch("/", data = "<collection_to_update>")]
pub async fn update_collection(
    conn: DbConn,
    collection_to_update: Json<Collection>,
) -> ApiResponse {
    let collection_to_update = collection_to_update.0;

    let result = conn
        .run(move |c| {
            diesel::update(collections::table.filter(collections::id.eq(collection_to_update.id)))
                .set(collections::name.eq(collection_to_update.name))
                .get_result::<Collection>(c)
        })
        .await;

    match result {
        Ok(updated_collection) => ApiResponse::new(json!(updated_collection), Status::Accepted),
        Err(Error::NotFound) => ApiResponse::new_message("Collection not found", Status::NotFound),
        Err(error) => ApiResponse::new_message(
            &format!("Unable to patch collection: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use rocket::serde::json::serde_json::json;

    #[test]
    fn test_collections() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.get("/collections").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_json(), Some(json!([])));
    }
}
