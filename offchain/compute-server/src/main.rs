use std::{
    time::Duration,
    sync::Arc,
    env,
};

use ethers::{
    utils::Anvil,
};

use tokio::sync::Mutex;

use cartesi_compute_server::{
    config::{
        ArenaConfig,
        ContractArtifactsConfig,
        EngineConfig,
        APIServerConfig,
    },
    arena::EthersArena,
    machine::MachineFactory,
    engine::Engine,
    server::APIServer,
}; 

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arena_config = ArenaConfig{
        web3_http_url: String::from("http://localhost:8545"),
        private_key: String::from("dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"),
        contract_artifacts: ContractArtifactsConfig { 
            single_level_factory: String::from("compute-server/artifacts/SingleLevelTournamentFactory.json"), 
            top_factory: String::from("compute-server/artifacts/TopTournamentFactory.json"), 
            middle_factory: String::from("compute-server/artifacts/MiddleTournamentFactory.json"), 
            bottom_factory: String::from("compute-server/artifacts/BottomTournamentFactory.json"), 
            tournament_factory: String::from("compute-server/artifacts/TournamentFactory.json"),
        },
    };
    let mut arena = EthersArena::new(arena_config)?;
    arena.init().await?;
    let arena = Arc::new(arena);

    // Create and initialize machine factory.
    let machine_factory = MachineFactory::new(9545).await?;
    let machine_factory = Arc::new(Mutex::new(machine_factory));

    // Create and initialize engine.
    let engine_config = EngineConfig{
        player_react_period: Duration::from_secs(5),
    };
    let engine = Arc::new(Engine::<EthersArena>::new(arena, machine_factory, engine_config));

    // Create and run API server.
    let server_config = APIServerConfig {
        address: String::from("[::1]:50051"), 
    };
    let server = APIServer::new(engine, server_config);
    server.run().await
}