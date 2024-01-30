use sqlx::{
    Error, 
    PgPool
};
use super::model::Character;

pub async fn create(character: &Character, conn: PgPool) -> Result<(), Error> {
    let query = "INSERT INTO characters (name) VALUES ($1)";

    sqlx::query(query)
        .bind(&character.name)
        .execute(&conn)
        .await?;

    Ok(())
}

pub async fn update(character: &Character, conn: PgPool) -> Result<(), Error> {
    let query = "UPDATE characters SET current_node_id = $1 WHERE id = $2";
    
    sqlx::query(query)
        .bind(&character.current_node_id)
        .bind(&character.id)
        .execute(&conn)
        .await?;
    
    Ok(())
}