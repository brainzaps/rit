use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about = "Git implementation in Rust", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Initialize a new git repository")]
    Init,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
    }
}
