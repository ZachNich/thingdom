use axum::{
    extract::State, 
    Json,
    http::StatusCode
};
use crate::state::model::AppState;
use super::{
    model::Character, 
    repository
};

pub async fn create(State(app_state): State<AppState>, Json(character): Json<Character>) -> Result<(), StatusCode> {
    if let Ok(()) = repository::create(&character, app_state.db_connection).await {
        Ok(())
    } else {
        Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
}

pub async fn update(State(app_state): State<AppState>, Json(character): Json<Character>) -> Result<(), StatusCode> {
    match repository::update(&character, app_state.db_connection).await {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("{:#?}", e);
            return Err(StatusCode::UNPROCESSABLE_ENTITY)
        }
    };
}