use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use url::Url;

mod format;
mod sync;

#[derive(Parser, Clone, Debug)]
struct Synchronizer {
    #[clap(subcommand)]
    command: Command,
}

impl Synchronizer {
    async fn run() -> Result<()> {
        let synchronizer = Synchronizer::parse();

        synchronizer.run_command().await
    }

    async fn run_command(self) -> Result<()> {
        match self.command {
            Command::Sync {
                data_path,
                engine_url,
            } => {
                sync::sync(data_path, engine_url).await?;
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

        #[clap(
            short,
            long,
            value_parser,
            value_name = "ENGINE_URL",
            env = "ENGINE_URL"
        )]
        engine_url: Url,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "env-file")]
    {
        dotenvy::dotenv().ok();
    }

    Synchronizer::run().await
}
