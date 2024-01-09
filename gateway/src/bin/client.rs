//! L0-Gateway Command Line Interface & Client

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{info, error, warn, debug};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {

    /// Set a custom config file.
    #[arg(short, long, value_name = "FILE_PATH")]
    config: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {}

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliArgs::parse();
    
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    println!("Hello, world!");
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    tracing::trace!("trace");

    Ok(())
}

