use axum::{Router, routing::{get, post}};
use amigo_secreto::handlers::*;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/game", get(game))
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/", ServeFile::new("index.html"));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}