use std::{
    time::Duration,
    sync::Arc,
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
    let web3_rpc_url = "http://localhost:8545";
    let arena_sec_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    let machine_rpc_url = "http://machine:5002";
    let rpc_address = "0.0.0.0:50500";
    
    let arena_config = ArenaConfig{
        web3_http_url: String::from(web3_rpc_url),
        private_key: String::from(arena_sec_key),
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
    let machine_factory = MachineFactory::new(String::from(machine_rpc_url)).await?;
    let machine_factory = Arc::new(Mutex::new(machine_factory));

    // Create and initialize engine.
    let engine_config = EngineConfig{
        player_react_period: Duration::from_secs(5),
    };
    let engine = Arc::new(Engine::<EthersArena>::new(arena, machine_factory, engine_config));

    // Create and run API server.
    let server_config = APIServerConfig {
        address: String::from(rpc_address), 
    };
    let server = APIServer::new(engine, server_config);
    server.run().await
}