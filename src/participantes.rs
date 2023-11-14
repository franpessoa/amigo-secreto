use std::path::Path;
use serde::{Deserialize, Serialize};

/// Struct that represents a game participant
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Participante {
    #[serde(rename = "Nome")]
    pub nome: String,
    #[serde(rename = "Email")]
    pub email: String
}

// Internal representation of JSON document
#[derive(Serialize, Deserialize, Debug)]
pub struct Jogo {
    pub participantes: Vec<Participante>,
    pub seed: Option<String>
}

/// Reads the participants of a game from a JSON file
pub fn read_participants(path: &Path) 
    -> Result<Vec<Participante>, std::io::Error>
{
    let mut results = vec![];

    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() 
    {
        let record: Participante = result?;
        results.push(record)
    }

    return Ok(results)
}