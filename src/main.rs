use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod format;
mod sync;

#[derive(Parser)]
struct Synchronizer {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Format {
        #[clap(short, long, value_parser, value_name = "PATH")]
        data_path: PathBuf,
    },
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
        Command::Format { data_path } => {
            format::format(data_path).unwrap();
        }
    }
}
