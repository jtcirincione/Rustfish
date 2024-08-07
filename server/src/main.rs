use std::sync::{Arc, Mutex};
pub mod routes;
pub mod game_state;

#[tokio::main]
async fn main() {
    println!("hi, world!");

    let state = Arc::new(Mutex::new(game_state::GameState::new()));
    let app = routes::create_routes(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();    
    axum::serve(listener, app).await.unwrap();
}

