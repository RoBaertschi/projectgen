use std::env;

use std::path::PathBuf;

use parsers::args::Args;
use parsers::args::Commands;

mod commands;
pub mod parsers;

pub struct State {
    command: Commands,
    _cwd: PathBuf,
}

impl State {
    pub fn build(args: Args) -> Result<Self, std::io::Error> {
        let command = args.command;
        let cwd: PathBuf = match command {
            Commands::New { ref path, .. } => path.to_path_buf(),
            Commands::Init { .. } => std::env::current_dir()?,
        };

        Ok(State { command, _cwd: cwd })
    }
}

pub fn run(state: State) -> Result<(), String> {
    match state.command {
        Commands::New { path, template } => {
            if path.exists() {
                return Err(format!("Path already exists."));
            }

            match std::fs::create_dir(&path) {
                Err(err) => {
                    return Err(format!(
                        "An Error occurred while creating a Directory. Error: {}",
                        err
                    ))
                }
                _ => (),
            }

            if let Err(err) = env::set_current_dir(&path) {
                return Err(format!(
                    "Error while changing into the following directory: {:?}. With Error: {}",
                    path, err
                ));
            }

            commands::init_directory(template)
        }
        Commands::Init { template } => commands::init_directory(template),
    }
}
