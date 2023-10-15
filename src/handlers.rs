use axum::Json;
use rand::seq::SliceRandom;
use crate::{participants::{Game, Participant}, rng::gen_rng, email::send};
use serde::Serialize;

pub async fn root() -> String {
    return String::from("Hello world")
}

#[derive(Debug, Serialize)]
pub struct GameSuccess {
    seed: String,
    players: Vec<Participant>
}

#[derive(Debug, Serialize)]
pub struct GameError {
    error: String
}

impl GameError {
    fn new(error: String) -> Self {
        return GameError { error }
    }
}

impl GameSuccess {
    fn new(seed: String, players: Vec<Participant>) -> Self {
        return GameSuccess { seed, players }
    }
}

pub async fn game(Json(data): Json<Game>) -> Result<Json<GameSuccess>, Json<GameError>> {
    let (mut rng, seed) = gen_rng(data.seed);
    let mut players = data.participants;

    players.shuffle(&mut rng);

    let mut handles = vec![];
    for (idx, player) in players.iter().enumerate() {
        let selected = match players.get(idx + 1) {
            Some(p) => &p.name,
            None => &(&players).get(0).unwrap().name
        };

        let email_result = send(
            format!("{} <{}>", player.name, player.email), 
            selected.to_owned()
        );
        
        handles.push(tokio::spawn(email_result))
    }

    let results = futures::future::join_all(handles).await;
    for j in results {
        match j {
            Err(e) => return Err(Json(GameError::new(e.to_string()))),
            Ok(_) => continue
        }
    }

    return Ok(Json(GameSuccess::new(seed, players)))
}