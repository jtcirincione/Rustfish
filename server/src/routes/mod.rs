use axum::{
    routing::get, 
    Router,
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::{Arc, Mutex};
use crate::game_state::GameState;
use serde_json::json;


pub fn create_routes(state: Arc<Mutex<GameState>>) -> Router {
    Router::new()
        .route("/board", get(get_board))
        .with_state(state)
}

async fn get_board(State(state): State<Arc<Mutex<GameState>>>) -> impl IntoResponse {
    //NOTE: may cause error because of signage in js
    let game_state = state.lock().unwrap();
    return Json(json!({
        "game": game_state.game_to_array()
    }));
}

async fn make_move(State(state): State<Arc<Mutex<GameState>>>) -> impl IntoResponse {
    let game_state = state.lock().unwrap();
    
    return (StatusCode::OK, String::from("Successfully moved!"))
}