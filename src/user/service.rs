use axum::{
    http::StatusCode, 
    Json, 
    extract::State
};
use crate::state::model::AppState;
use super::{
    model::User, 
    repository
};

pub async fn get_all(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    if let Ok(users) = repository::get_all(app_state.db_connection).await {
        Ok(Json(users))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn create(State(app_state): State<AppState>, Json(user): Json<User>) -> Result<(), StatusCode> {
    if let Ok(()) = repository::create(&user, app_state.db_connection).await {
        Ok(())
    } else {
        Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
}