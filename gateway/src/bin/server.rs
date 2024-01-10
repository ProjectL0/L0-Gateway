//! L0 Gateway Server

use gateway::server::{Server, config::ServerConfig};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new(ServerConfig::default())?;
    server.serve().await?;
    Ok(())
}

