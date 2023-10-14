use std::{path::Path, fs};
use serde::{Deserialize, Serialize};

/// Struct that represents a game participant
#[derive(Serialize, Deserialize, Debug)]
pub struct Participant {
    pub name: String,
    pub email: String
}

// Internal representation of JSON document
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub participants: Vec<Participant>,
    pub seed: Option<String>
}

/// Reads the participants of a game from a JSON file
pub fn read_participants(json_path: &Path) 
    -> Option<Vec<Participant>> 
{
    let file = fs::read_to_string(json_path).ok()?;
    let data: Game = serde_json::from_str(&file).ok()?;

    return Some(data.participants)
}