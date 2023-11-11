use std::path::Path;
use clap::Parser;
use amigo_secreto::participants::read_participants;
use amigo_secreto::rng::gen_rng;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use rand::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long)]
    data: String,

    #[arg(short, long)]
    seed: Option<String>
}

fn main() {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let path =  Path::new(&args.data);
    
    let mut participants = read_participants(path).unwrap();
    
    let (mut rng, seed) = gen_rng(None);
    println!("Sorteio com chave : {}", seed);

    participants.shuffle(&mut rng);
    println!("Resultado : {:?}", &participants);

    
}
