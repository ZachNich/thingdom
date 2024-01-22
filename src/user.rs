use sqlx::{PgConnection, error::BoxDynError, Error, FromRow, types::Uuid};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<Uuid>,
    pub email: String
}

pub async fn create(user: &User, mut conn: PgConnection) -> Result<(), BoxDynError> {
    let query = "INSERT INTO users (email) VALUES ($1)";

    sqlx::query(query)
        .bind(&user.email)
        .execute(&mut conn)
        .await?;
    
    Ok(())
}

pub async fn get_all(mut conn: PgConnection) -> Result<Vec<User>, Error> {
    let query = "SELECT * FROM USERS";
    
    let users = sqlx::query_as::<_, User>(query)
        .fetch_all(&mut conn)
        .await?;

    Ok(users)
}