use crate::models::CollectionsDbConn;
use rocket::{get, post};

#[get("/<collection>")]
pub async fn get_collection_points(conn: CollectionsDbConn, collection: String) -> String {
    conn.run(|connection| println!("Querying DB")).await;
    format!(
        "On construction! We will return all pins for collection {}",
        collection
    )
}

#[post("/<collection>")]
pub fn post_collection(collection: String) -> String {
    format!(
        "On construction! This will create a new collection named{}",
        collection
    )
}
