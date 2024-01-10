//! L0 Gateway Server

use std::net::{TcpListener, SocketAddr};

use tracing::info;

use crate::rpc::{GatewayRPC, self, gateway_rpc::gateway_server::GatewayServer};

use self::{config::ServerConfig};

pub mod config;

pub const DEFAULT_LISTEN_PORT: u16 = 8810;

pub struct Server {
    config: ServerConfig,
    local_addr: SocketAddr,
}

impl Server {
    pub fn new(config: ServerConfig) -> anyhow::Result<Self> {
        let local_addr = SocketAddr::from(([127,0,0,1], config.listen_port));
        Ok(Self { 
            config,
            local_addr,
        })
    }

    pub async fn serve(&mut self) -> anyhow::Result<()> {
        let gateway_rpc = GatewayRPC::default();
        info!("Gateway server listening on {}", self.local_addr);
        tonic::transport::Server::builder()
            .add_service(GatewayServer::new(gateway_rpc))
            .serve(self.local_addr)
            .await?;
        Ok(())
    }
}
