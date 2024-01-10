//! Ethereum Service

use std::{net::SocketAddr};

use config::EthServiceConfig;
use rpc::{eth_rpc::{eth_server::EthServer}, EthRPC};
use tonic::transport::{server::Router};

pub mod config;
pub mod rpc;

pub const DEFAULT_ETH_API_PORT: u16 = 8820;
pub const DEFAULT_ETH_P2P_PORT: u16 = 8821;

pub struct EthService {
    config: EthServiceConfig,
    p2p_addr: SocketAddr,
}

impl EthService {
    pub fn new(config: EthServiceConfig) -> Self {
        // TODO: impl security
        Self { 
            config: config.clone(),
            p2p_addr: SocketAddr::from(([127,0,0,1], config.p2p_port)),
        }
    } 

    pub fn build_p2p(&mut self) -> Router {
        tonic::transport::Server::builder().add_service(EthServer::new(EthRPC::default()))
    }
}
