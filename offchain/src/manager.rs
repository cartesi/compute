use std::{
    error::Error, default,
};

use tonic::{Request, Response, Status};

use crate::{
    grpc:: {
        StartDisputeRequest,
        StartDisputeResponse,
        FinishDisputeRequest,
        FinishDisputeResponse,
        GetDisputeInfoRequest,
        GetDisputeInfoResponse,
        JoinDisputeRequest,
        DisputeInfo,
        Compute, JoinDisputeResponse
    },
    config::PlayerConfig,
    arena::Arena,
};

pub struct ComputeManager {
    arena: Box<dyn Arena>,
    player_config: PlayerConfig,
}

impl ComputeManager {
    pub fn new(arena: Box<dyn Arena>, player_config: PlayerConfig) -> ComputeManager {
        Self {
            arena: arena,
            player_config: player_config,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.arena.init().await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl Compute for ComputeManager {    
    async fn start_dispute(
        &self,
        request: Request<StartDisputeRequest>,
    ) -> Result<Response<StartDisputeResponse>, Status> {

        Ok(Response::new(StartDisputeResponse{ dispute_id: String::default() }))
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
