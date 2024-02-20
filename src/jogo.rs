use rand::seq::SliceRandom;

use crate::{email::iter_send, participantes::Participante, rng::gen_rng};

#[derive(Debug, Clone)]
pub struct Jogo {
    participantes: Vec<Participante>,
    seed: Option<String>
}

pub struct JogoResultado {
    pub seed: String,
    pub emails: Vec<Option<lettre::transport::smtp::response::Response>>,
    pub participantes: Vec<Participante>
}

impl Jogo {
    pub fn novo(prealloc_participantes: usize) -> Self {
        return Self {
            participantes: Vec::with_capacity(prealloc_participantes),
            seed: None
        }
    }

    pub fn add_participante(&mut self, participante: Participante) {
        self.participantes.push(participante)
    }

    pub fn realizar_jogo(&mut self) -> JogoResultado {
        let (mut rng, seed) = gen_rng(self.seed.clone());
        self.participantes.shuffle(&mut rng);

        assert!(self.participantes.len() > 1); // não faz o jogo com menos de dois participantes

        return JogoResultado {
            seed,
            emails: iter_send(self.participantes.clone()),
            participantes: self.participantes.clone(),
        }
    }
}