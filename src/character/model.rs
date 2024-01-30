use sqlx::{
    types::Uuid,
    FromRow
};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Character {
    pub id: Option<Uuid>,
    pub name: String,
    pub current_node_id: Option<Uuid>
}