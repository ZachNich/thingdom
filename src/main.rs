
use axum::{
    routing::get,
    Router,
    Json,
    http::StatusCode
};
use crate::user::{User, create, get_all};
use sqlx::{PgConnection, Connection, migrate::MigrateError, Error, error::BoxDynError};

mod user;

#[tokio::main]
async fn main() -> Result<(), BoxDynError> {

    // postgresql://[YOUR_USERNAME]:[YOUR_PASSWORD]@[YOUR_HOST_NAME]:[YOUR_PORT]/[DATABASE_NAME]
    let uri = "postgresql://postgres:postgres@localhost:5432/thingdom";
    let mut conn = connect(uri).await?;
    migrate(&mut conn).await?;

    // let new_user = User {
    //     id: None,
    //     email: "e@mydomain.com".to_string()
    // };
    // create(&new_user, conn).await?;

    let users = get_all(conn).await?;
    let iter = users.into_iter().next();
    
    match iter {
        Some(user) => {
            
            let handler = || async { Ok::<Json<User>, (StatusCode, String)>(Json(user)) };
            let app = Router::new().route("/", get(handler));
            
            axum::Server::bind(&"0.0.0.0:1337".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        },
        None => {

        }
    }

    Ok(())
}

async fn connect(uri: &str) -> Result<PgConnection, Error> {
    let conn = sqlx::postgres::PgConnection::connect(uri).await?;
    Ok(conn)
}

async fn migrate(conn: &mut PgConnection) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations")
        .run(conn)
        .await?;
    Ok(())
}