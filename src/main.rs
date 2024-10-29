#![allow(unused_imports)]
use std::fs::{self, rename, DirBuilder};

use clap::{Args, Parser, Subcommand, ValueEnum};
use log::*;
use otter::parse_toml;

/// Doc comment
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Subcommand)]
enum Command {
    Run,
    Verbose,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Command::Run => {
            parse_toml().unwrap();
        }
        Command::Verbose => {}
    }
}
