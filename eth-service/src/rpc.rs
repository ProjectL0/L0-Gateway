use tonic::{Request, Response, Status};

pub mod eth_rpc {
    tonic::include_proto!("eth");
}

use eth_rpc::eth_server::{Eth, EthServer};
use eth_rpc::{Ping, Pong};

#[derive(Default)]
pub struct EthRPC {}

#[tonic::async_trait]
impl Eth for EthRPC {
    async fn hello(&self, request: Request<Ping>) -> Result<Response<Pong>, Status> {
        tracing::info!("got ping from {:?}", request.remote_addr());
        let res = eth_rpc::Pong { payload: request.into_inner().payload };
        Ok(Response::new(res))
    }
}
