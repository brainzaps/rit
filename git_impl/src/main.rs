use clap::{Args, Parser, Subcommand};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about = "Git implementation in Rust", long_about = None)]
struct GitArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Initialize a new git repository")]
    Init(InitArgs),
}

#[derive(Args, Debug)]

struct InitArgs {
    #[arg(short = 'p', long = "path")]
    path: Option<String>,
}

fn main() {
    let args = GitArgs::parse();

    match args.command {
        Command::Init(args) => {
            if let Some(path) = args.path {
                print!("Creating git repository at {}", path);
            }

            // fs::create_dir(".git").unwrap();
            // fs::create_dir(".git/objects").unwrap();
            // fs::create_dir(".git/refs").unwrap();
            // fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            // println!("Initialized git directory")
        }
    }
}
