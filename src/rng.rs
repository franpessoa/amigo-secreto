use rand::{prelude::*, distributions::Alphanumeric};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

pub fn gen_rng(seed: Option<String>) -> (ChaCha20Rng, String) {
    let rng_seed = match seed {
        Some(s) => s,
        None => {
            let seed: String = ChaCha20Rng::from_entropy()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();
            
            seed
        }
    };

    return (Seeder::from(&rng_seed).make_rng(), rng_seed.clone())
}