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

// USER //
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

// CHARACTER //
#[derive(Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: Option<Uuid>,
    pub name: String,
    pub current_node_id: Option<Uuid>
}

pub async fn update_character(character: &Character, conn: PgPool) -> Result<(), Error> {
    let query = "UPDATE CHARACTERS SET current_node_id = (current_node_id) WHERE ID = (id) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&character.current_node_id)
        .bind(&character.id)
        .execute(&conn)
        .await?;
    
    Ok(())
}

pub async fn create_character(character: &Character, conn: PgPool) -> Result<(), Error> {
    let query = "INSERT INTO characters (name) VALUES ($1)";

    sqlx::query(query)
        .bind(&character.name)
        .execute(&conn)
        .await?;

    Ok(())
}

// ADVENTURE SPOT //
#[derive(Serialize, Deserialize, FromRow)]
pub struct AdventureSpot {
    pub id: Option<Uuid>,
    pub name: String
}

// ADVENTURE NODE //
#[derive(Serialize, Deserialize)]
pub enum NodeType {
    Story,
    Combat
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AdventureNode {
    pub id: Option<Uuid>,
    pub node_type: NodeType,
    pub body: String
}

// ADVENTURE OPTION //
#[derive(Serialize, Deserialize, FromRow)]
pub struct AdventureOption {
    pub id: Option<Uuid>,
    pub body: String,
    pub next_node_id: Uuid
}