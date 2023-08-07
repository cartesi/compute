use std::{
    error::Error,
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
        DisputeInfo,
        Compute
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

        Ok(Response::new(StartDisputeResponse{}))
    }

    async fn finish_dispute(
        &self,
        request: Request<FinishDisputeRequest>,
    ) -> Result<Response<FinishDisputeResponse>, Status> {
        Ok(Response::new(FinishDisputeResponse{}))
    }

    async fn get_dispute_info(
        &self,
        request: Request<GetDisputeInfoRequest>,
    ) -> Result<Response<GetDisputeInfoResponse>, Status> {
        Ok(Response::new(GetDisputeInfoResponse{
            info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }
}
