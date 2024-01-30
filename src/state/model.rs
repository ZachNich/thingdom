use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: PgPool
}