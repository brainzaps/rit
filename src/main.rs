use camino::Utf8PathBuf;
use clap::Parser;
use serde::Serialize;
use std::fmt;

/// Scaffold a new post for your blog
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, default_value_t, value_enum)]
    rit: Command,

    #[clap(short, long, default_value = "content")]
    output_dir: Utf8PathBuf,
}
fn main() {
    let args = Args::parse();
    dbg!(&args);

    print!("{}", args.rit);
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Command {
    #[default]
    Init,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Init => write!(f, "INIT"),
        }
    }
}
