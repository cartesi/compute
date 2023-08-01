use std::{
    error::Error,
    sync::Arc,
};

use ethers::{
    types::{Bytes, Address},
    providers::{Provider, Http},
    signers::LocalWallet,
    contract:: ContractFactory,
    middleware::SignerMiddleware,
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

use crate::contract::{
    INNER_TOURNAMENT_FACTORY_ABI,
    INNER_TOURNAMENT_FACTORY_BYTECODE,
    ROOT_TOURNAMENT_FACTORY_ABI,
    ROOT_TOURNAMENT_FACTORY_BYTECODE,
    RootTournamentFactory,
};

use crate::config::Config;

pub type Web3Provider = Provider<Http>; 
pub type Web3Client = SignerMiddleware<Web3Provider, LocalWallet>;

#[derive(Debug)]
pub struct ComputeManager {
    config: Config,
    provider: Web3Provider,
    client: Arc<Web3Client>,
    root_factory_address: Address,
}

impl ComputeManager {
    pub fn new(config: &Config) -> ComputeManager {
        let wallet: LocalWallet = config.wallet_private_key.parse::<LocalWallet>()
            .expect("failed to init wallet");
        let provider = Provider::<Http>::try_from(config.web3_http_url.clone())
            .expect("failed to init web3 provider");        
        let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

        Self {
            config: config.clone(),
            provider: provider,
            client,
            root_factory_address: Address::default(),
        }
    }
    
    pub async fn deploy_contracts(&mut self) -> Result<(), Box<dyn Error>> {
        // Deploy inner tournament factory.
        let inner_factory_abi = (*INNER_TOURNAMENT_FACTORY_ABI).clone();
        let inner_factory_bytecode = Bytes::from(INNER_TOURNAMENT_FACTORY_BYTECODE.clone());
        let inner_factory_deployer = ContractFactory::new(inner_factory_abi, inner_factory_bytecode, self.client.clone());
        let inner_factory = inner_factory_deployer
            .deploy(())?
            .confirmations(0usize)
            .send()
            .await?;

        // Deploy root tournament factory.
        let root_factory_abi = (*ROOT_TOURNAMENT_FACTORY_ABI).clone();
        let root_factory_bytecode = Bytes::from(ROOT_TOURNAMENT_FACTORY_BYTECODE.clone());
        let root_factory_deployer = ContractFactory::new(root_factory_abi, root_factory_bytecode, self.client.clone());
        let root_factory = root_factory_deployer
            .deploy(inner_factory.address())?
            .confirmations(0usize)
            .send()
            .await?; 

        self.root_factory_address = root_factory.address();
        
        return Ok(());
    }
}

#[tonic::async_trait]
impl Compute for ComputeManager {    
    async fn start_dispute(
        &self,
        request: Request<StartDisputeRequest>,
    ) -> Result<Response<StartDisputeResponse>, Status> {
        let root_factory = RootTournamentFactory::new(self.root_factory_address, self.client.clone());
        root_factory.instantiate_top_of_multiple(initial_hash).send().await;

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
