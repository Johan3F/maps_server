use axum::Router;
use dotenv::dotenv;
use std::{env, net::SocketAddr};

mod domain;

mod db;
use db::get_db_pool;

mod api;
use api::handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    read_environment();

    let db_pool = get_db_pool(&env::var("DATABASE_URL")?)
        .await
        .expect("unable to get a db connection pool");

    // build our application with a route
    let app = Router::new().nest("/collections", handlers::collections::add_routes(db_pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);
    // run it
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn read_environment() {
    dotenv().ok();

    let expected_env_variables = vec!["DATABASE_URL"];

    for expected_env_variable in expected_env_variables {
        let expect_message = format!("{expected_env_variable} expected to be set");
        let _ = env::var(expected_env_variable).expect(&expect_message);
    }
}
