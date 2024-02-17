mod cli;
#[cfg(feature = "copy")]
mod clipboard;
pub mod config;
mod kitchen;
mod reply;
mod storage;

use clap::Parser;
use cli::args::{Args, Commands};
use cli::choose::choose;
use cli::setup::setup;

use storage::{clean_dir, CONFIG_DIR};

fn main() {
    let cli = Args::parse();
    let command = cli.command.unwrap_or(Commands::Choose(cli.choose));

    match command {
        Commands::Choose(args) => {
            choose(args).unwrap();
        }
        Commands::Setup(args) => {
            setup(args).unwrap();
        }
        Commands::Clean => {
            clean_dir(&CONFIG_DIR).expect("Could not clean config");
        }
    }
}
