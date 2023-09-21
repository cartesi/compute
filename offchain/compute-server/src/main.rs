use std::{
    time::Duration,
    sync::Arc,
};

use cartesi_compute_server::{
    config::{
        ArenaConfig,
        ContractArtifactsConfig,
        EngineConfig,
        APIServerConfig,
    },
    arena::EthersArena,
    engine::Engine,
    server::APIServer,
}; 

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and initialize arena.
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
    let mut arena = EthersArena::new(arena_config);
    arena.init().await?;
    let arena = Arc::new(arena);

    // Create and initialize engine.
    let engine_config = EngineConfig{
        player_react_period: Duration::from_secs(5),
    };
    let engine = Arc::new(Engine::<EthersArena>::new(arena, engine_config));

    // Create and run API server.
    let server_config = APIServerConfig {
        address: String::from("[::1]:50051"), 
    };
    let server = APIServer::new(engine, server_config);
    server.run().await
}