use std::path::Path;
use clap::Parser;
use amigo_secreto::participants::{read_participants};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long)]
    data: String
}

fn main() {
    let args = Args::parse();
    let path =  Path::new(&args.data);
    
    let participants = read_participants(path)
        .unwrap();

    println!("{:#?}", participants);
}
