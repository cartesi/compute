use std::sync::Arc;

use ethers::abi::AbiEncode;
use tonic::{
    transport::Server,
    Request,
    Response,
    Status,
};

use crate::{
    grpc:: {
        StartDisputeRequest,
        StartDisputeResponse,
        FinishDisputeRequest,
        FinishDisputeResponse,
        GetDisputeInfoRequest,
        GetDisputeInfoResponse,
        JoinDisputeRequest,
        JoinDisputeResponse,
        DisputeInfo,
        Compute, 
        ComputeServer,
    },
    merkle::Hash,
    arena::{Address, Arena},
    engine::Engine,
    config::APIServerConfig,
};

pub struct APIServer<A: Arena> {
    engine: Arc<Engine<A>>,
    config: APIServerConfig,
}

impl<A: Arena + 'static> APIServer<A> {
    pub fn new(engine: Arc<Engine<A>>, config: APIServerConfig) -> Self {
        Self {
            engine: engine,
            config: config,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: start engine and server in concurrent tasks and wait for them
        let server_addr = self.config.address.parse()?;
        Server::builder()
            .add_service(ComputeServer::new(self))
            .serve(server_addr)
            .await?;
        Ok(())
    }

    async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
        // TODO: stop engine and server
        Ok(())
    }
}

#[tonic::async_trait]
impl<A: Arena + 'static> Compute for APIServer<A> {    
    async fn start_dispute(
        &self,
        request: Request<StartDisputeRequest>,
    ) -> Result<Response<StartDisputeResponse>, Status> {
        let req = request.into_inner(); 
        
        if !is_valid_digest_data(req.initial_hash) {
            return Err(Status::invalid_argument("invalid initial hash digest"))
        }
        let initial_hash = Hash::from_digest_data(req.initial_hash);
        
        match self.engine.start_dispute(initial_hash, req.snapshot_path).await {
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
        
        match self.engine.finish_dispute(root_tournament).await {
            Ok(dispute_state) => Ok(
                Response::new(FinishDisputeResponse{
                    dispute_info: Some(DisputeInfo{
                        closed: dispute_state.finished,
                    }),
                })
            ),
            Err(err) => Err(Status::internal(err.to_string())),
        }
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

        match self.engine.disupte_state(root_tournament).await {
            Some(state) => Ok(Response::new(GetDisputeInfoResponse{
                dispute_info: Some(DisputeInfo {
                    closed: false,
                }),
            })),
            None => Err(Status::internal("dispute not found")),
        }
    }

    async fn join_dispute(
        &self,
        request: Request<JoinDisputeRequest>,
    ) -> Result<Response<JoinDisputeResponse>, Status> {
        let req = request.into_inner();

        let root_tournament = if let Ok(tournament) = req.dispute_id.parse::<Address>() {
            tournament
        } else {
            return Err(Status::invalid_argument("invalid dispute tournament address"))
        };

        match self.engine.create_player(root_tournament).await {
            Ok(_) => Ok(Response::new(JoinDisputeResponse{
                dispute_info: Some(DisputeInfo {
                    closed: false,
                }),
            })),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }
}

fn is_valid_digest_data(digest_data: Vec<u8>) -> bool {
    digest_data.len() == 32
}