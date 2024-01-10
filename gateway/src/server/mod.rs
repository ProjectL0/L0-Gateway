//! L0 Gateway Server

use std::net::{TcpListener, SocketAddr};

use self::{config::ServerConfig, rpc::GatewayRPC};

pub mod config;
mod rpc;

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
        println!("Gateway server listening on {}", self.local_addr);
        tonic::transport::Server::builder()
            .add_service(rpc::gateway_rpc::gateway_server::GatewayServer::new(gateway_rpc))
            .serve(self.local_addr)
            .await?;
        Ok(())
    }
}
