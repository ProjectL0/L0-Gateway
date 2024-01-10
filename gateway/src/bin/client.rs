//! L0-Gateway Command Line Interface & Client

use std::{path::PathBuf, net::SocketAddr};

use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use gateway::client::{Client, ClientConfig};
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Spin up a gateway server.
    Up {
        /// 
        local_addr: Option<SocketAddr>,

        /// Path to a custom configuration file.
        #[arg(short, long, value_name = "FILE_PATH")]
        config: Option<PathBuf>,
    },

    /// Spin down a gateway server.
    Down {},

    /// Attach to an already existing gateway server.
    Attach {
        local_addr: SocketAddr,
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliArgs::parse();
    
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Some(Commands::Up { local_addr, config }) => {
            match config {
                Some(path) => {
                    info!("starting server with custom configs");
                    tokio::process::Command::new("l0gatewayd")
                        .arg("start")
                        .arg("--config")
                        .arg(format!("{}", path.display()))
                        .output().await.context("unable to create child process")?;
                },
                None => {
                    info!("starting server with default configs");
                    tokio::process::Command::new("l0gatewayd")
                        .arg("start")
                        .output().await.context("unable to create child process")?;
                },
            }
            match local_addr {
                Some(local_addr) => {
                    let mut client = Client::new(ClientConfig::new(local_addr)).await?;
                },
                None => {
                    let mut client = Client::new(ClientConfig::default()).await?;
                }
            }
        },
        Some(Commands::Down {  }) => unimplemented!(),
        Some(Commands::Attach { local_addr }) => {
            let mut client = Client::new(ClientConfig::new(local_addr)).await?;
        },
        None => {
            return Err(anyhow!("missing arguments"))?;
        },
    }

    Ok(())
}
