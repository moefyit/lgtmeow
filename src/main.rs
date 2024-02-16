mod cli;
#[cfg(feature = "copy")]
mod clipboard;
mod config;
mod kitchen;
mod reply;
mod setup;
mod storage;

use clap::Parser;
use cli::Cli;
use config::Config;
use kitchen::partial_data::get_partial_metadata;
use rand::prelude::SliceRandom;
use storage::{clean_dir, CONFIG_DIR};

fn main() {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(cli::Commands::Choose(cli.choose));

    match command {
        cli::Commands::Choose(args) => {
            if !Config::exists() {
                eprintln!("Please run `lgtmeow setup` first.");
                return;
            }
            let metadata = get_partial_metadata();
            let config = Config::load().unwrap();
            let replies = reply::load_saved_replies_from_config(config, &metadata);
            let reply;
            if args.random {
                reply = replies.choose(&mut rand::thread_rng()).unwrap();
                eprintln!("{}", reply.title);
                println!("{}", reply.content);
            } else {
                eprintln!("Currently only random is supported.");
                return;
            }
            if args.copy {
                #[cfg(not(feature = "copy"))]
                {
                    panic!("Copy feature is not enabled, please recompile with `--features copy`");
                }
                #[cfg(feature = "copy")]
                {
                    clipboard::copy_to_clipboard(&reply.content).unwrap();
                }
            }
        }
        cli::Commands::Setup(args) => {
            setup::setup(args).unwrap();
        }
        cli::Commands::Clean => {
            clean_dir(&CONFIG_DIR).expect("Could not clean config");
        }
    }
}
