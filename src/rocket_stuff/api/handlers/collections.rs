use rocket::{
    delete, get,
    http::Status,
    patch, post, routes,
    serde::{
        json::{serde_json::json, Json},
        uuid::Uuid,
    },
    Build, Rocket,
};

use crate::{
    api::models::ApiResponse,
    db::DbConn,
    domain::collections::{CollectionNoID, CollectionsController, Error},
};

pub fn add_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/collections",
        routes![
            get_collection,
            post_collection,
            get_collections,
            update_collection,
            delete_collection,
        ],
    )
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

#[post("/", format = "application/json", data = "<new_collection>")]
pub async fn post_collection(conn: DbConn, new_collection: Json<CollectionNoID>) -> ApiResponse {
    match CollectionsController::create_collection(conn, new_collection.into_inner()).await {
        Ok(collection) => ApiResponse::new(json!(collection), Status::Created),
        Err(Error::AlreadyExists {
            name: existing_collection,
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

#[get("/<collection_id>")]
pub async fn get_collection(conn: DbConn, collection_id: Uuid) -> ApiResponse {
    match CollectionsController::get_collection(conn, collection_id).await {
        Ok(collections) => ApiResponse::new_ok(json!(collections)),
        Err(Error::NotFound { id: id_not_found }) => ApiResponse::new_message(
            &format!("Collection '{id_not_found}' not found"),
            Status::NotFound,
        ),
        Err(error) => ApiResponse::new_message(
            &format!("unable to get collections: {error:?}"),
            Status::InternalServerError,
        ),
    }
}

#[patch(
    "/<collection_id>",
    format = "application/json",
    data = "<modified_collection>"
)]
pub async fn update_collection(
    conn: DbConn,
    collection_id: Uuid,
    modified_collection: Json<CollectionNoID>,
) -> ApiResponse {
    let modified_collection = modified_collection.0;

    match CollectionsController::update_collection(conn, collection_id, modified_collection).await {
        Ok(updated_collection) => ApiResponse::new(json!(updated_collection), Status::Ok),
        Err(Error::NotFound { id: id_not_found }) => ApiResponse::new_message(
            &format!("Collection '{id_not_found}' not found"),
            Status::NotFound,
        ),
        Err(error) => ApiResponse::new_message(
            &format!("unable to update collection: {error:?}"),
            Status::InternalServerError,
        ),
    }
}

#[delete("/<collection_id>")]
pub async fn delete_collection(conn: DbConn, collection_id: Uuid) -> ApiResponse {
    match CollectionsController::delete_collection(conn, collection_id).await {
        Ok(()) => ApiResponse::new(json!({}), Status::Accepted),
        Err(Error::NotFound { id: id_not_found }) => {
            ApiResponse::new_message(&format!("id '{id_not_found}' not found"), Status::NotFound)
        }
        Err(error) => ApiResponse::new_message(
            &format!("Unable to remove collection: {}", error),
            Status::InternalServerError,
        ),
    }
}

#[cfg(test)]
mod test {
    use rocket::{
        http::{Status, ContentType},
        local::blocking::Client,
        serde::{json::serde_json::to_string, uuid::Uuid},
    };

    use crate::{
        domain::collections::{Collection, CollectionNoID},
        rocket,
    };

    #[test]
    fn test_collections() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let collection_to_insert = CollectionNoID {
            name: format!("test_collection_{}", Uuid::new_v4()),
        };

        // Adding a new collection
        let response = client
            .post("/collections")
            .header(ContentType::new("application", "json"))
            .body(to_string(&collection_to_insert).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        let inserted_collection = response.into_json::<Collection>().unwrap();
        assert_eq!(collection_to_insert.name, inserted_collection.name);

        // Verifying that the collection was added
        let response = client
            .get(format!("/collections/{}", inserted_collection.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let stored_collection = response.into_json::<Collection>().unwrap();
        assert_eq!(inserted_collection.id, stored_collection.id);
        assert_eq!(inserted_collection.name, stored_collection.name);

        // Update collection
        let modified_collection = Collection {
            id: inserted_collection.id,
            name: format!("modified_{}", inserted_collection.name),
        };
        let response = client
            .patch(format!("/collections/{}", inserted_collection.id))
            .header(ContentType::new("application", "json"))
            .body(to_string(&modified_collection).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let got_modified_collection = response.into_json::<Collection>();
        assert!(got_modified_collection.is_some());
        assert_eq!(
            got_modified_collection.unwrap().name,
            modified_collection.name
        );

        // Remove collection
        let response = client
            .delete(format!("/collections/{}", inserted_collection.id))
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
    }
}
