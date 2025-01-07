use clap::Parser;
use cli::Cli;

mod cli;
mod configs;

fn main() {
    let cli = Cli::parse();
}
