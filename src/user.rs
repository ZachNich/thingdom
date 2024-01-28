use sqlx::{
    types::Uuid, 
    Error, 
    FromRow, 
    PgPool
};
use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<Uuid>,
    pub email: String
}

pub async fn create(user: &User, conn: PgPool) -> Result<(), Error> {
    let query = "INSERT INTO users (email) VALUES ($1)";

    sqlx::query(query)
        .bind(&user.email)
        .execute(&conn)
        .await?;
    
    Ok(())
}

pub async fn get_all(conn: PgPool) -> Result<Vec<User>, Error> {
    let query = "SELECT * FROM USERS";
    
    let users = sqlx::query_as::<_, User>(query)
        .fetch_all(&conn)
        .await?;

    Ok(users)
}