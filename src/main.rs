use axum::{Router, routing::{post, get_service}};
use amigo_secreto::handlers::*;
use hyper::{Request, Body, StatusCode};
use tower_http::{services::{ServeDir, ServeFile}, cors::{Any, CorsLayer}};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();


    let app = Router::new()
        .route("/game", post(game))
        .nest_service("/", ServeDir::new("public").append_index_html_on_directories(true))
        .layer(CorsLayer::new().allow_origin(Any));

    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}