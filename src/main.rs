
use axum::{
    http::StatusCode, 
    routing::{
        get, 
        post
    }, 
    Json, 
    Router,
    extract::State
};
use user::{
    create, create_character, get_all, Character, User
};
use sqlx::{
    migrate::MigrateError, 
    postgres::PgPoolOptions, 
    Error, 
    PgPool
};

mod user;

#[derive(Clone)]
struct AppState {
    db_connection: PgPool
}

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

async fn migrate(conn: PgPool) -> Result<(), MigrateError> {
    //will initialize your database with scripts in migrations directory
    sqlx::migrate!("./migrations")
        .run(&conn)
        .await?;
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
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
        .with_state(state);
    Ok(router)
}

async fn get_all_users(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    if let Ok(users) = get_all(app_state.db_connection).await {
        Ok(Json(users))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn create_user(State(app_state): State<AppState>, Json(user): Json<User>) -> Result<(), StatusCode> {
    if let Ok(()) = create(&user, app_state.db_connection).await {
        Ok(())
    } else {
        Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
}

async fn create_character(State(app_state): State<AppState>, Json(character): Json<Character>) -> Result<(), StatusCode> {
    if let Ok(()) = create_character(&character, app_state.db_connection).await {
        Ok(())
    } else {
        Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
}