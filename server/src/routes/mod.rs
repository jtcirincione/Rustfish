use axum::{
    routing::get, 
    routing::post,
    Router,
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::{Arc, Mutex};
use crate::game_state::GameState;
use serde_json::json;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct RequestPayload {
    from: u64,
    to: u64,
    piece_type: String
}


pub fn create_routes(state: Arc<Mutex<GameState>>) -> Router {
    Router::new()
        .route("/board", get(get_board))
        .route("/move", post(make_move))
        .with_state(state)
}

async fn get_board(State(state): State<Arc<Mutex<GameState>>>) -> impl IntoResponse {
    //NOTE: may cause error because of signage in js
    let game_state = state.lock().unwrap();
    return Json(json!({
        "game": game_state.game_to_array()
    }));
}

async fn make_move(State(state): State<Arc<Mutex<GameState>>>, Json(payload): Json<RequestPayload>) -> impl IntoResponse {
    let mut game_state = state.lock().unwrap();
    let RequestPayload {to, from, piece_type} = payload;
    game_state.make_move(from, to, &piece_type);
    return (StatusCode::OK, String::from("Successfully moved!"))
}