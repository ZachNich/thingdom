use sqlx::{
    types::Uuid,
    FromRow,
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