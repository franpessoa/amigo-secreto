pub mod pages;
pub mod db;
pub mod sorteio;

use std::collections::HashMap;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{
    response::Html, routing::get, 
    Router, extract::*
};
use sqlx::mysql::MySqlQueryResult;
use sqlx::{Pool, MySql};
use tower_http::trace;
use tracing::Level;
use crate::pages::*;
use crate::db::*;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    dotenv::dotenv().unwrap();
    
    let pool = pool_init().await;

    let router = Router::new()
        .route("/db/jogadores", get(get_jogadores_handler).post(post_jogadores_handler))
        .route("/db/jogos", get(get_jogos_handler).post(post_jogos_handler))
        .with_state(pool)
        // Add tracing and logging
        .layer(trace::TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
            .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
        );

    let server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service());
        
    tracing::info!("Starting server at {}", server.local_addr());
    server.await.unwrap();
}

async fn get_jogadores_handler(State(pool): State<Pool<MySql>>, Query(params): Query<HashMap<String, String>>) -> Result<Json<Vec<OutJogador>>, (StatusCode, String)> {
    let id_raw = params.get("id");
    
    if id_raw.is_none() {
        return Err((StatusCode::BAD_REQUEST, String::from("E necessario um ID")))
    } else {
        let id = id_raw.unwrap().parse::<u32>();
        
        if id.is_err() {
            return Err(map_internal_err(id.err().unwrap()))
        } else {
            let jogadores = get_jogadores(id.unwrap(), &pool).await?;
            return Ok(axum::response::Json(jogadores))
        }
    }
}

async fn get_jogos_handler(State(pool): State<Pool<MySql>>, Query(params): Query<HashMap<String, String>>) -> Result<Json<OutJogo>, (StatusCode, String)> {
    let id_raw = params.get("id");
    
    if id_raw.is_none() {
        return Err((StatusCode::BAD_REQUEST, String::from("necessario um ID")))
    } else {
        let id = id_raw.unwrap().parse::<u32>();
        
        if id.is_err() {
            return Err(map_internal_err(id.err().unwrap()))
        } else {
            let jogo = get_jogo(id.unwrap(), &pool).await?;
            return Ok(axum::response::Json(jogo))
        }
    }
}

async fn post_jogos_handler(State(pool): State<Pool<MySql>>, Json(data): Json<Jogo>) ->
    Result<Json<CreateApiResponse>, (StatusCode, String)> 
{
        let res = add_jogo(data, &pool).await?;
        tracing::info!("{:?}", res);
        
        return Ok(Json(CreateApiResponse {
            status: true,
            id: res.last_insert_id(),
            rows_affected: res.rows_affected()
        }))
}

async fn post_jogadores_handler(State(pool): State<Pool<MySql>>, Json(data): Json<Jogador>) ->
    Result<Json<CreateApiResponse>, (StatusCode, String)> 
{
        let res = add_jogador(data, &pool).await?;
        tracing::info!("{:?}", res);
        
        return Ok(Json(CreateApiResponse {
            status: true,
            id: res.last_insert_id(),
            rows_affected: res.rows_affected()
        }))
}
/*
#[cfg(test)]
mod tests {
    use crate::{db::OutJogador, sorteio::sortear};

    #[test]
    fn test_sorteio() {
        let mut jogadores = vec![];
        for i in 1..16 {
            jogadores.push(OutJogador {
                nome: format!("f{}", i).to_string(),
                email: "franciscomspessoa@gmail.com".to_string(),
                id_jogador: i
            })
        }
        
        let new_jogadores = sortear(&jogadores);
        for i in new_jogadores.keys() {
            println!("{} tirou {}", jogadores[i.clone() as usize].nome, jogadores[new_jogadores.get(i).unwrap().clone() as usize].nome)
        }
    }
}*/