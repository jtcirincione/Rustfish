use std::sync::{Arc, Mutex};
pub mod routes;
pub mod game_state;
mod utils {
    pub mod moves {
        pub mod king_moves;
        pub mod knight_moves;
        pub mod pawn_moves;
        pub mod sliding_piece_moves;
        pub mod bishop_moves;
        pub mod rook_moves;
        pub mod queen_moves;
    }
}

#[tokio::main]
async fn main() {
    println!("hi, world!");
    let state = Arc::new(Mutex::new(game_state::GameState::new()));
    let app = routes::create_routes(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();    
    axum::serve(listener, app).await.unwrap();
}

