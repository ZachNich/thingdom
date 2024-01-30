mod state;
mod user;
mod character;

use axum::{
    routing::{
        get, 
        put, 
        post
    }, 
    Router
};
use sqlx::{
    postgres::PgPoolOptions, 
    Error, 
};
use state::model::AppState;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "0.0.0.0:1337".parse().unwrap();
    let router = create_router().await?;
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn create_router() -> Result<Router, Error> {
    // postgresql://[YOUR_USERNAME]:[YOUR_PASSWORD]@[YOUR_HOST_NAME]:[YOUR_PORT]/[DATABASE_NAME]
    let uri = "postgresql://postgres:postgres@localhost:5432/thingdom";
    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(uri)
        .await?;
    let state = AppState { db_connection };
    let router = Router::new()
        .route("/users", get(user::service::get_all))
        .route("/users", post(user::service::create))
        .route("/characters", post(character::service::create))
        .route("/characters", put(character::service::update))
        .with_state(state);
    Ok(router)
}