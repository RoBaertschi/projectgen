use clap::Parser;
use projectgen::{parsers::args::Args, State};

fn main() {
    let state = State::build(Args::parse());

    match state {
        Ok(state) => projectgen::run(state).unwrap(),
        Err(error) => eprintln!("{}", error),
    }
}
