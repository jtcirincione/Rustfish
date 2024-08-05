use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use std::sync::{Arc, Mutex};
pub mod routes;
pub mod game_state;

#[tokio::main]
async fn main() {
    println!("hi, world!");

    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin, or specify a specific origin like `http://localhost:3000`
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);
    let state = Arc::new(Mutex::new(game_state::GameState::new()));
    let app = routes::create_routes(state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();    
    axum::serve(listener, app).await.unwrap();
}

