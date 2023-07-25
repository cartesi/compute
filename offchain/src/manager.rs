use std::{
    error::Error,
    boxed::Box,
    sync::Arc,
};

use tonic::{Request, Response, Status};
use crate::grpc:: {
    StartDisputeRequest,
    StartDisputeResponse,
    FinishDisputeRequest,
    FinishDisputeResponse,
    GetDisputeInfoRequest,
    GetDisputeInfoResponse,
    DisputeInfo,
    Compute,
};

use ethers::{
    prelude::abigen,
    providers::{Provider, Http},
    signers::LocalWallet,
    middleware::SignerMiddleware,
};

use crate::config::Config;
use crate::dispute::Dispute;

abigen!(InnerTournamentFactory, "src/abi/InnerTournamentFactory.json");
abigen!(RootTournamentFactory, "src/abi/RootTournamentFactory.json");


#[derive(Debug)]
pub struct ComputeManager {
    config: Config,
    provider: Provider<Http>,
}

impl ComputeManager {
    pub fn new(config: &Config) -> ComputeManager {
        let provider = Provider::<Http>::try_from(&config.web3_http_url)
            .expect("failed to init web3 provider");
        
        Self {
            config: config.clone(),
            provider: provider,
        }
    }
    
    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        let wallet: LocalWallet = self.config.wallet_private_key.parse::<LocalWallet>()
            .expect("failed to init wallet");
        let client = Arc::new(SignerMiddleware::new(&self.provider, wallet));
        return Ok(())
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
