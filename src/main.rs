// use axum::{
//     routing::get,
//     Router,
// };

use sqlx::Connection;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // axum::Server::bind(&"0.0.0.0:1337".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // postgresql://[YOUR_USERNAME]:[YOUR_PASSWORD]@[YOUR_HOST_NAME]:[YOUR_PORT]/[DATABASE_NAME]
    let uri = "postgresql://postgres:postgres@localhost:5432/thingdom";
    let mut conn = sqlx::postgres::PgConnection::connect(uri).await?;
    sqlx::query("CREATE TABLE demoTable ( demoCol int )").execute(&mut conn).await?;

    Ok(())
}