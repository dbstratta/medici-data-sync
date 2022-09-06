use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod data;
mod raw_data;
mod sync;

#[derive(Parser)]
struct Synchronizer {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Sync {
        #[clap(short, long, value_parser, value_name = "PATH")]
        data_path: PathBuf,
    },
}

fn main() {
    let synchronizer = Synchronizer::parse();

    match synchronizer.command {
        Command::Sync { data_path } => {
            sync::sync(data_path).unwrap();
        }
    }
}
