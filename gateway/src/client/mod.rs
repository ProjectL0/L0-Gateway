//! L0 Gateway Client

use std::net::SocketAddr;

use tracing::info;

use crate::{server::DEFAULT_LISTEN_PORT, rpc::gateway_rpc::{gateway_client::GatewayClient, Ping}};

pub struct Client {
    local_addr: SocketAddr,
}

impl Default for Client {
    fn default() -> Self {
        Client::new(SocketAddr::from(([127,0,0,1], DEFAULT_LISTEN_PORT)))
    }
}

impl Client {
    pub fn new(local_addr: SocketAddr) -> Self {
        Self {
            local_addr,
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let mut gateway_client = GatewayClient::connect(format!("http://{}", self.local_addr)).await?;
        let req = tonic::Request::new(Ping {
            payload: "Hi Mom!".into()
        });
        info!("request = {:?}", req);
        let res = gateway_client.hello(req).await?;
        info!("response = {:?}", res);
        Ok(())
    }
}
