use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod format;
mod sync;

#[derive(Parser, Clone, Debug)]
struct Synchronizer {
    #[clap(subcommand)]
    command: Command,
}

impl Synchronizer {
    fn run() -> Result<()> {
        let synchronizer = Synchronizer::parse();

        synchronizer.run_command()
    }

    fn run_command(self) -> Result<()> {
        match self.command {
            Command::Sync { data_path } => {
                sync::sync(data_path)?;
            }
            Command::Format { data_path } => {
                format::format(data_path)?;
            }
        }

        Ok(())
    }
}

#[derive(Subcommand, Clone, Debug)]
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
    Synchronizer::run().unwrap();
}
