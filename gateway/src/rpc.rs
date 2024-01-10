use tonic::{Request, Response, Status};

pub mod gateway_rpc {
    tonic::include_proto!("gateway");
}

use gateway_rpc::gateway_server::{Gateway, GatewayServer};
use gateway_rpc::{Ping, Pong};
use tracing::info;

#[derive(Default)]
pub struct GatewayRPC {}

#[tonic::async_trait]
impl Gateway for GatewayRPC {
    async fn hello(&self, request: Request<Ping>) -> Result<Response<Pong>, Status> {
        info!("got ping from {:?}", request.remote_addr());
        let res = gateway_rpc::Pong {
            payload: request.into_inner().payload,
        };
        Ok(Response::new(res))
    }
}
