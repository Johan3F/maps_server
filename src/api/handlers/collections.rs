use diesel::{prelude::*, result::Error};
use rocket::{
    delete, get,
    http::Status,
    patch, post,
    serde::json::{serde_json::json, Json},
};
use uuid::Uuid;

use crate::{
    api::models::ApiResponse,
    db::{schema::collections, DbConn},
    domain::{
        controllers::collections::{CollectionsController, CollectionsError},
        models::collection::{Collection, CollectionNew},
    },
};

#[post("/", data = "<new_collection>")]
pub async fn post_collection(conn: DbConn, new_collection: Json<CollectionNew>) -> ApiResponse {
    match CollectionsController::create_collection(conn, new_collection.into_inner()).await {
        Ok(collection) => ApiResponse::new(json!(collection), Status::Created),
        Err(CollectionsError::AlreadyExists {
            existing: existing_collection,
        }) => ApiResponse::new_message(
            &format!("Collection '{existing_collection}' already exists"),
            Status::AlreadyReported,
        ),
        Err(error) => ApiResponse::new_message(
            &format!("unable to create new collection: {error:?}"),
            Status::InternalServerError,
        ),
    }
}

#[get("/")]
pub async fn get_collections(conn: DbConn) -> ApiResponse {
    match CollectionsController::get_collections(conn).await {
        Ok(collections) => ApiResponse::new_ok(json!(collections)),
        Err(error) => ApiResponse::new_message(
            &format!("unable to get collections: {error:?}"),
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
        Ok(updated_collection) => ApiResponse::new(json!(updated_collection), Status::Ok),
        Err(Error::NotFound) => ApiResponse::new_message("Collection not found", Status::NotFound),
        Err(error) => ApiResponse::new_message(
            &format!("Unable to patch collection: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use rocket::{http::Status, local::blocking::Client, serde::json::serde_json::to_string};

    use crate::{
        domain::models::collection::{Collection, CollectionNew},
        rocket,
    };

    #[test]
    fn test_collections() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let collection_to_insert = CollectionNew {
            name: format!("test_collection_{}", Uuid::new_v4()),
        };

        // Adding a new collection
        let response = client
            .post("/collections")
            .body(to_string(&collection_to_insert).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);

        // Verifying that the collection was added
        let response = client.get("/collections").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let stored_collections = response.into_json::<Vec<Collection>>().unwrap();
        assert!(stored_collections.len() >= 1);
        let inserted_collection: Vec<_> = stored_collections
            .iter()
            .enumerate()
            .filter_map(|(_, collection)| {
                if collection.name == collection_to_insert.name {
                    return Some(collection);
                }
                None
            })
            .collect();
        assert_eq!(inserted_collection.len(), 1);
        let inserted_collection = inserted_collection[0];
        assert_eq!(inserted_collection.name, collection_to_insert.name);

        // Update collection
        let modified_collection = Collection {
            id: inserted_collection.id,
            name: format!("modified_{}", inserted_collection.name),
        };
        let response = client
            .patch("/collections")
            .body(to_string(&modified_collection).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let got_modified_collection = response.into_json::<Collection>();
        assert!(got_modified_collection.is_some());
        assert_eq!(
            got_modified_collection.unwrap().name,
            modified_collection.name
        );
    }
}
