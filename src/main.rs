use std::path::{PathBuf, Path};

use clap::Parser;
use projectgen::parsers::args::{self, Commands, Args};

struct State {
    command: Commands,
    cwd: PathBuf,
}

impl State {
    fn build(args: Args) -> Result<Self, std::io::Error> {

        let command = args.command;
        let cwd: PathBuf = match command {
            Commands::New { ref path, .. } => path.to_path_buf(),
            Commands::Init { .. } => std::env::current_dir()?,
        };

        Ok(State { command, cwd })
    }
}

fn main() {
    let state = State::build(Args::parse());

    match state {
        Ok(state) => unimplemented!(),
        Err(error) => eprintln!("{}", error),
    }
}
