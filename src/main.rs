#![warn(rust_2018_idioms, clippy::all, clippy::pedantic)]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
}

fn main() {
    env_logger::init();
    Cli::parse();
}
