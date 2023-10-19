use std::{
    sync::Arc,
    time::Duration,
};

use tokio::sync::Mutex;

use cartesi_compute_core::{
    arena::{ArenaConfig, ContractArtifactsConfig, EthersArena},
    machine::MachineFactory,
}; 
use cartesi_compute_coordinator::grpc::CoordinatorClient;

use cartesi_compute_simulator::engine::{EngineConfig, Engine};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simple_linux_program = "/data/programs/simple-linux-program";
    let simple_program = "/data/programs/simple-program";

    let coordinator_address = "0.0.0.0:50500";
    let web3_rpc_url = "http://localhost:8545";
    let web3_sec_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    let machine_rpc_host= "http://machine";
    let machine_rpc_port = 5002;
  
    let arena_config = ArenaConfig{
        web3_http_url: String::from(web3_rpc_url),
        private_key: String::from(web3_sec_key),
        contract_artifacts: ContractArtifactsConfig { 
            single_level_factory: String::from("core/artifacts/SingleLevelTournamentFactory.json"), 
            top_factory: String::from("core/artifacts/TopTournamentFactory.json"), 
            middle_factory: String::from("core/artifacts/MiddleTournamentFactory.json"), 
            bottom_factory: String::from("core/artifacts/BottomTournamentFactory.json"), 
            tournament_factory: String::from("core/artifacts/TournamentFactory.json"),
        },
    };
    let arena = EthersArena::new(arena_config)?;
    let arena = Arc::new(arena);

    let coordinator = CoordinatorClient::connect(coordinator_address).await?;
    let coordinator = Arc::new(Mutex::new(coordinator));
    
    let machine_factory = MachineFactory::new(
        String::from(machine_rpc_host),
        machine_rpc_port
    ).await?;
    let machine_factory = Arc::new(Mutex::new(machine_factory));

    let engine_config = EngineConfig{
        player_react_period: Duration::from_secs(5),
    };
    let engine = Arc::new(Engine::<EthersArena>::new(
        coordinator,
        arena,
        machine_factory,
        engine_config
    ));

    Ok(())
}