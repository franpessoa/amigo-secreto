use std::path::Path;
use clap::Parser;
use amigo_secreto::participantes::read_participants;
use amigo_secreto::rng::gen_rng;
use rand::prelude::*;
use amigo_secreto::email::iter_send;
use chrono;
use colored::Colorize;

extern crate prettytable;
use prettytable::{Table, row};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long)]
    data: String,

    #[arg(short, long)]
    seed: Option<String>
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let path =  Path::new(&args.data);
    
    let mut participants = read_participants(path)
        .unwrap();
    
    let (mut rng, seed) = gen_rng(args.seed);
    participants.shuffle(&mut rng);

    let mut table = Table::new();
    table.add_row(row!["Semente", seed]);
    table.add_row(row!["Horário", chrono::offset::Local::now()]);
    println!("{}","Sorteio: ".bold());
    table.printstd();
    println!("\n");

    let rs = iter_send(participants.clone()).await;
    let mut result_table = Table::new();
    result_table.add_row(row!["N°", "Nome", "Resultado"]);
    for (idx, i) in rs.iter().enumerate() {

        result_table.add_row(row![idx, participants.get(idx).unwrap().nome, format!("{:?}", i)]);
        
    }

    result_table.printstd();
}
