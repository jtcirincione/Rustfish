use axum::{
    routing::get,
    Router,
    http::Method,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    println!("hi, world!");

    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin, or specify a specific origin like `http://localhost:3000`
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);


    let app = Router::new()
        .route("/board", get(get_board))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();    
    axum::serve(listener, app).await.unwrap();
}

async fn get_board() -> String {
    return String::from("jefflovescockandballs world!");
}
