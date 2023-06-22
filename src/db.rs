use std::time::Duration;
use axum::http::StatusCode;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlQueryResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Jogo {
    nome: String,
    max: u8,
    senha: String
}

#[derive(Debug, Deserialize)]
pub struct Jogador {
    nome: String,
    email: String,
    jogo: u32,
    senha: String
}

#[derive(sqlx::FromRow, Serialize, Clone, Debug, PartialEq)]
pub struct OutJogador {
    pub nome: String,
    pub email: String,
    pub id_jogador: i32
}

#[derive(sqlx::FromRow, Serialize)]
pub struct OutJogo {
    nome: String,
    max: i32,
    senha: String,
    id_jogo: i32,
    data: String
}

#[derive(Serialize)]
pub struct CreateApiResponse {
    pub status: bool,
    pub id: u64,
    pub rows_affected: u64
}

pub async fn pool_init() -> MySqlPool {
    let pool = MySqlPoolOptions::new().max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
        
    return pool
}

pub async fn add_jogo(jogo: Jogo, pool: &MySqlPool) -> Result<MySqlQueryResult, (StatusCode, String)> {
    sqlx::query!("INSERT INTO jogos(nome, senha, max) VALUES (?, ?, ?);", jogo.nome, jogo.senha, jogo.max)    
        .execute(pool)
        .await
        .map_err(map_internal_err)
}

pub async fn add_jogador(jogador: Jogador, pool: &MySqlPool) -> Result<MySqlQueryResult, (StatusCode, String)> {
    let jogo = get_jogo(jogador.jogo, pool).await?;
    let jogadores = get_jogadores(jogador.jogo, pool).await?;
    
    if jogo.senha != jogador.senha {
        return Err((StatusCode::UNAUTHORIZED, String::from("Senha errada")))
    } else if jogadores.len()as i32 >= jogo.max {
        return Err((StatusCode::FORBIDDEN, String::from("Maximo de jogadores excedido")))
    } else {
        return sqlx::query!("INSERT INTO jogadores(nome, email, id_jogo) VALUES (?, ?, ?);", jogador.nome, jogador.email, jogador.jogo)
            .execute(pool)
            .await
            .map_err(map_internal_err)
    }
}

pub async fn get_jogo(id: u32, pool: &MySqlPool) -> Result<OutJogo, (StatusCode, String)> {
    sqlx::query_as!(OutJogo, "SELECT * FROM jogos WHERE jogos.id_jogo = ?", id)
        .fetch_one(pool)
        .await
        .map_err(map_internal_err)
}

pub async fn get_jogadores(id: u32, pool: &MySqlPool) -> Result<Vec<OutJogador>, (StatusCode, String)> {
    sqlx::query_as!(OutJogador, "SELECT jogadores.nome, jogadores.email, jogadores.id_jogador FROM jogadores INNER JOIN jogos ON jogos.id_jogo = jogadores.id_jogo AND jogos.id_jogo = ?;", id)
        .fetch_all(pool)
        .await
        .map_err(map_internal_err)
}

pub fn map_internal_err<E>(err: E) -> (StatusCode, String) where E: std::error::Error, {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string()
    )
}