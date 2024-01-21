use sqlx::{PgConnection, error::BoxDynError};

pub(crate) struct User {
    pub id: Option<uuid::Uuid>,
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