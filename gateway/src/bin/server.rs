//! L0 Gateway Server

use std::path::PathBuf;

use clap::Parser;
use gateway::server::{Server, config::ServerConfig};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    #[arg(short, long, value_name = "FILE_PATH")]
    config: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliArgs::parse();

    let mut server = Server::new(ServerConfig::default())?;
    server.serve().await?;
    Ok(())
}

