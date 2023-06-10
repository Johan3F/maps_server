use axum::Router;
use std::net::SocketAddr;

mod api;
use api::handlers;

mod db;
use db::get_db_pool;

const DATABASE_URL: &str = "postgres://username:password@postgis:5432/postgres";

#[tokio::main]
async fn main() {
    let _db_pool = get_db_pool(DATABASE_URL)
        .await
        .expect("unable to get a db connection pool");

    // build our application with a route
    let app = Router::new()
        .nest("/collections", handlers::collections::add_routes())
        .with_state(());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);
    // run it
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
