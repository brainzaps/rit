use clap::{Args, Parser, Subcommand};
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

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
            let current_dir = env::current_dir().unwrap();

            let path = match args.path {
                Some(p) => p,
                None => current_dir.to_str().unwrap().to_string(),
            };

            let absolute_path = fs::canonicalize(path).unwrap();

            let root_path = Path::new(&absolute_path);

            let git_path = root_path.join(".git");

            for dir in ["objects", "refs"].iter() {
                let dir_path = git_path.join(dir);
                let result = fs::create_dir_all(&dir_path);

                if result.is_err() {
                    println!("fatal: {}", result.err().unwrap());
                    exit(1);
                }
            }

            println!("Initialized empty rit repository in {}", git_path.display());

            // if let Some(path) = args.path {
            //     print!("Creating git repository at {}", path);
            // }

            // fs::create_dir(".git").unwrap();
            // fs::create_dir(".git/objects").unwrap();
            // fs::create_dir(".git/refs").unwrap();
            // fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            // println!("Initialized git directory")
        }
    }
}
