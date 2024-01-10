//! Ethereum Service

use config::EthServiceConfig;

pub mod config;

pub const DEFAULT_ETH_API_PORT: u16 = 8820;
pub const DEFAULT_ETH_P2P_PORT: u16 = 8821;

pub struct EthService {
    config: EthServiceConfig,
}

impl EthService {
    pub fn new(config: EthServiceConfig) -> Self {
        Self { 
            config,
        }
    } 

    pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
