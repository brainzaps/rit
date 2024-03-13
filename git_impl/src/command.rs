use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "Git implementation in Rust", long_about = None)]
pub struct GitArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Initialize a new git repository")]
    Init(InitArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(short = 'p', long = "path")]
    pub path: Option<String>,
}
