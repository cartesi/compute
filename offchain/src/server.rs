use std::time::Duration;

use tonic::transport::Server;

use cartesi_compute::{
    config::{
        ArenaConfig,
        ContractArtifactsConfig,
        PlayerConfig,
    },
    arena::EthersArena,
    engine::Engine,
    grpc::ComputeServer,
}; 

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arena_config = ArenaConfig{
        web3_http_url: String::from("http://localhost:8545"),
        private_key: String::from("dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"),
        contract_artifacts: ContractArtifactsConfig { 
            single_level_factory: String::new(), 
            top_factory: String::new(), 
            middle_factory: String::new(), 
            bottom_factory: String::new(), 
            tournament_factory: String::new(),
        },
    };
    let arena = Box::new(EthersArena::new(arena_config));

    let player_config = PlayerConfig{
        react_period: Duration::from_secs(5),
    };
    let compute = Engine::new(arena, player_config);

    println!("Starting gRPC Server...");

    let server_addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(ComputeServer::new(compute))
        .serve(server_addr)
        .await?;

    Ok(())
}