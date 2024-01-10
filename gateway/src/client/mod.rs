//! L0 Gateway Client

use std::net::SocketAddr;

use tonic::transport::Channel;
use tracing::info;

use crate::{server::DEFAULT_LISTEN_PORT, rpc::gateway_rpc::{gateway_client::GatewayClient, Ping}};

pub struct ClientConfig {
    pub local_addr: SocketAddr,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig::new(SocketAddr::from(([127,0,0,1], DEFAULT_LISTEN_PORT)))
    }
}

impl ClientConfig {
    pub fn new(local_addr: SocketAddr) -> Self {
        Self {
            local_addr,
        }
    }
}

pub struct Client {
    config: ClientConfig,
    gateway_client: GatewayClient<Channel>,
}

impl Client {
    pub async fn new(config: ClientConfig) -> anyhow::Result<Self> {
        // TODO: impl security, tls, https, etc
        let mut gateway_client = GatewayClient::connect(format!("http://{}", config.local_addr)).await?;
        let req = tonic::Request::new(Ping {
            payload: "Hi Mom!".into()
        });
        info!("request = {:?}", req);
        let res = gateway_client.hello(req).await?;
        info!("response = {:?}", res);
        Ok(Self { config, gateway_client })
    }
}
