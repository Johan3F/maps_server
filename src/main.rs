use axum::{response::Json, routing::get, Router};
use serde::Serialize;

use std::net::SocketAddr;

mod api;
use api::handlers;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(helloworld))
        .nest("/collections", handlers::add_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);
    // run it
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn helloworld() -> Json<Hello> {
    Json(Hello {
        message: "Hello world!".to_owned(),
    })
}

#[derive(Serialize)]
struct Hello {
    message: String,
}
