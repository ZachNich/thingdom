// use axum::{
//     routing::get,
//     Router,
// };
mod user;
use crate::user::{User, create};
use sqlx::{PgConnection, Connection, migrate::MigrateError, Error, error::BoxDynError};

#[tokio::main]
async fn main() -> Result<(), BoxDynError> {
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // axum::Server::bind(&"0.0.0.0:1337".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // postgresql://[YOUR_USERNAME]:[YOUR_PASSWORD]@[YOUR_HOST_NAME]:[YOUR_PORT]/[DATABASE_NAME]
    let uri = "postgresql://postgres:postgres@localhost:5432/thingdom";
    let mut conn = connect(uri).await?;
    conn = migrate(conn).await?;

    let new_user = User {
        id: None,
        email: "myemail@mydomain.com".to_string()
    };
    create(&new_user, conn).await?;

    Ok(())
}

async fn connect(uri: &str) -> Result<PgConnection, Error> {
    let conn = sqlx::postgres::PgConnection::connect(uri).await?;
    Ok(conn)
}

async fn migrate(mut conn: PgConnection) -> Result<PgConnection, MigrateError> {
    sqlx::migrate!("./migrations")
        .run(&mut conn)
        .await?;
    Ok(conn)
}