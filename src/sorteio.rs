use std::collections::HashMap;

use crate::db::OutJogador;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn sortear(jogadores: &Vec<OutJogador>) -> HashMap<u32, u32>{
    let mut shuffled = (0..jogadores.len() as u32).collect::<Vec<u32>>();
    let mut results: HashMap<u32, u32> = HashMap::new();
    shuffled.shuffle(&mut thread_rng());
    
    for i in 0..jogadores.len() as u32 {
        results.insert(i, shuffled[i as usize]);
    }
    
    for i in &results {
        if i.0 == i.1 {
            return sortear(jogadores)
        }
    }
    
    return results
}