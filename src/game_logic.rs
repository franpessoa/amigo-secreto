use crate::participants::Participant;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

pub fn shuffle(input: &mut Vec<Participant>, seed: Option<String>) -> String {
    let mut rng: ChaCha20Rng = match seed {
        Some(x) => Seeder::from(x).make_rng(),
        None => {
            ChaCha20Rng::from_entropy()
        }
    };
    
    input.shuffle(&mut rng);
    return String::from_utf8(rng.get_seed().to_vec()).unwrap();
}