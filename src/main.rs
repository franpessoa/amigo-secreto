use std::path::Path;
use clap::Parser;
use amigo_secreto::{jogo::Jogo, participantes::read_participants};
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

fn main() {
    println!("Sorteio de amigo secreto");

    // lê variaveis de ambiente
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let path =  Path::new(&args.data);
    
    let participantes = read_participants(path)
        .unwrap();

    let mut jogo = Jogo::novo(participantes.len());
    
    for p in participantes {
        jogo.add_participante(p)
    }

    println!("Realizando sorteio");
    let resultado = jogo.realizar_jogo();

    let mut resultado_tabela = Table::new();
    resultado_tabela.add_row(row!["Nome", "Envio deu certo?"]);
    for (idx, email_resultado) in resultado.emails.iter().enumerate() {
        resultado_tabela.add_row(row![format!("{:?}", resultado.participantes[idx]), format!("{:?}", email_resultado.is_some())]);
    }

    resultado_tabela.printstd();

    println!("{}", format!("Semente utilizada: {}", resultado.seed).bold().green());

}
