use crate::{DEFAULT_ETH_API_PORT, DEFAULT_ETH_P2P_PORT};

#[derive(Clone)]
pub struct EthServiceConfig {
    pub api_port: u16,
    pub p2p_port: u16,
}

impl Default for EthServiceConfig {
    fn default() -> Self {
        Self { 
            api_port: DEFAULT_ETH_API_PORT,
            p2p_port: DEFAULT_ETH_P2P_PORT,
        }
    }
}
