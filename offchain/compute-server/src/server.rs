use std::sync::Arc;

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
    arena::Arena,
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
        match self.engine.start_dispute(Hash::from_data(req.initial_hash), req.snapshot_path).await {
            Ok(dispute_tournament) => Ok(Response::new(StartDisputeResponse{
                                            dispute_id: String::default()
                                        })),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn finish_dispute(
        &self,
        request: Request<FinishDisputeRequest>,
    ) -> Result<Response<FinishDisputeResponse>, Status> {
        Ok(Response::new(FinishDisputeResponse{ 
            dispute_info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }

    async fn get_dispute_info(
        &self,
        request: Request<GetDisputeInfoRequest>,
    ) -> Result<Response<GetDisputeInfoResponse>, Status> {
        Ok(Response::new(GetDisputeInfoResponse{
            dispute_info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }

    async fn join_dispute(
        &self,
        request: Request<JoinDisputeRequest>,
    ) -> Result<Response<JoinDisputeResponse>, Status> {
        Ok(Response::new(JoinDisputeResponse {
            dispute_info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }
}