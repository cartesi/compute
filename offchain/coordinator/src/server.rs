use std::sync::Arc;

use log::error;

use tokio::signal;
use tonic::{
    transport::Server,
    Request,
    Response,
    Status,
};
use ethers::abi::AbiEncode;

use cartesi_compute_core::{
    merkle::Hash,
    arena::{Address, Arena},
};

use crate::grpc:: {
        StartDisputeRequest,
        StartDisputeResponse,
        FinishDisputeRequest,
        FinishDisputeResponse,
        GetDisputeInfoRequest,
        GetDisputeInfoResponse,
        Coordinator, 
        CoordinatorServer,
    };

#[derive(Debug, Clone)]
pub struct APIServerConfig {
    pub address: String,
}

pub struct APIServer<A: Arena> {
    arena: Arc<A>,
    config: APIServerConfig,
}

impl<A: Arena + 'static> APIServer<A> {
    pub fn new(arena: Arc<A>, config: APIServerConfig) -> Self {
        Self {
            arena: arena,
            config: config,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> { 
        let server_addr = self.config.address.parse().unwrap();
        let server = Server::builder().add_service(CoordinatorServer::new(self));

        if let Err(err) = server.serve_with_shutdown(server_addr, async {
            if let Err(err) = signal::ctrl_c().await {
                error!("failed to catch ctrl-c signal - {}", err)
            }
        }).await {
            error!("failed to run grpc server - {}", err);
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl<A: Arena + 'static> Coordinator for APIServer<A> {    
    async fn start_dispute(
        &self,
        request: Request<StartDisputeRequest>,
    ) -> Result<Response<StartDisputeResponse>, Status> {
        let req = request.into_inner(); 
        
        if !is_valid_digest_data(&req.initial_hash) {
            return Err(Status::invalid_argument("invalid initial hash digest"))
        }
        let initial_hash = Hash::from_data(req.initial_hash);
        
        match self.arena.clone().create_root_tournament(initial_hash).await {
            Ok(dispute_tournament) => Ok(
                Response::new(StartDisputeResponse{
                    dispute_id: dispute_tournament.encode_hex(),
                })
            ),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn finish_dispute(
        &self,
        request: Request<FinishDisputeRequest>,
    ) -> Result<Response<FinishDisputeResponse>, Status> {
        let req = request.into_inner();
        
        let root_tournament = if let Ok(tournament) = req.dispute_id.parse::<Address>() {
            tournament
        } else {
            return Err(Status::invalid_argument("invalid dispute tournament address"))
        };
        
        todo!()
    }

    async fn get_dispute_info(
        &self,
        request: Request<GetDisputeInfoRequest>,
    ) -> Result<Response<GetDisputeInfoResponse>, Status> {
        let req = request.into_inner();

        let root_tournament = if let Ok(tournament) = req.dispute_id.parse::<Address>() {
            tournament
        } else {
            return Err(Status::invalid_argument("invalid dispute tournament address"))
        };

        todo!()
    }
}

fn is_valid_digest_data(digest_data: &Vec<u8>) -> bool {
    digest_data.len() == 32
}