use std::{path::Path, fs};
use serde::{Deserialize, Serialize};

/// Struct that represents a game participant
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Participant {
    pub nome: String,
    pub email: String
}

// Internal representation of JSON document
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub participants: Vec<Participant>,
    pub seed: Option<String>
}

/// Reads the participants of a game from a JSON file
pub fn read_participants(path: &Path) 
    -> Result<Vec<Participant>, std::io::Error>
{
    let mut results = vec![];

    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() 
    {
        let record: Participant = result?;
        results.push(record)
    }

    return Ok(results)
}