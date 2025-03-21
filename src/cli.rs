use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
#[command(infer_subcommands = true)]
pub struct Cli {
    #[arg(long, short)]
    /// The managers to run the command for
    pub managers: Option<Vec<String>>,
    #[arg(long, short)]
    /// Run all non-specified managers
    pub non_specified: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq)]
pub enum Commands {
    /// Build the current configuration
    Build,
    /// Print the difference between the system and the config
    Diff,
    /// Prints the currently active system config
    List,
    /// Upgrade all managers
    Upgrade,
}
