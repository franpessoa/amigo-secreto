use std::path::Path;
use serde::{Deserialize, Serialize};

/// Um único participante do jogo
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Participante {
    #[serde(rename = "Nome")]
    pub nome: String,
    #[serde(rename = "Email")]
    pub email: String
}

/// Reads the participants of a game from a JSON file
pub fn read_participants(path: &Path) 
    -> Result<Vec<Participante>, std::io::Error>
{
    let mut results = vec![];

    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() 
    {
        let mut record: Participante = result?;
        record.nome = record.nome.trim().to_string();
        results.push(record)
    }

    Ok(results)
}