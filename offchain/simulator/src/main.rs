use std::{
    sync::Arc,
    time::Duration,
    path::Path, result,
};

use tokio::{
    sync::Mutex,
    time,
};

use cartesi_compute_core::{
    merkle::Hash,
    arena::{ArenaConfig, ContractArtifactsConfig, EthersArena, Address},
    machine::MachineFactory,
}; 
use cartesi_compute_coordinator::grpc::CoordinatorClient;

use cartesi_compute_simulator::engine::{EngineConfig, Engine};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simple_linux_program = String::from("/data/programs/simple-linux-program");
    let simple_program = String::from("/data/programs/simple-program");

    let coordinator_address = "http://coordinator:50500";
    let machine_rpc_host= "http://machine";
    let machine_rpc_port = 5002;
  
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
        coordinator.clone(),
        machine_factory.clone(),
        engine_config
    ));

    let dispute_root_hash = dispute_root_hash(machine_factory.clone(), &simple_linux_program).await?; 
    
    let tournament = engine.clone().start_dispute(
        create_arena(String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))?,
        dispute_root_hash.into(),
        &simple_linux_program,
    ).await?;

    // Honest verifier.
    engine.clone().create_player(
        create_arena(String::from("0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"))?,
        false,
        tournament
    ).await?;

    // Malicious verifier.
    engine.clone().create_player(
        create_arena(String::from("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a"))?, 
        true,
        tournament
    ).await?;

    loop {
        engine.clone().execute_player(tournament, 0).await;
        engine.clone().execute_player(tournament, 1).await;
        time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

async fn dispute_root_hash(
    machine_factory: Arc<Mutex<MachineFactory>>,
    snapshot_path: &String,
) -> Result<Hash, Box<dyn std::error::Error>> {
    let mut machine_factory = machine_factory.lock().await;
    let machine = machine_factory.create_machine(Path::new(snapshot_path)).await?;
    let machine = machine.clone();
    let machine = machine.lock().await;
    Ok(machine.root_hash())
}

fn create_arena(web3_private_key: String) -> Result<Arc<EthersArena>, Box<dyn std::error::Error>>  {
    let web3_rpc_url = "http://anvil:8545";
    let web3_chain_id = 31337;
    let tournament_factory_address: Address = "0xdc64a140aa3e981100a9beca4e685f962f0cf6c9".parse()?;

    let arena_config = ArenaConfig{
        web3_rpc_url: String::from(web3_rpc_url),
        web3_private_key: String::from(web3_private_key),
        web3_chain_id: web3_chain_id,
        contract_artifacts: ContractArtifactsConfig { 
            single_level_factory: String::from("core/artifacts/SingleLevelTournamentFactory.json"), 
            top_factory: String::from("core/artifacts/TopTournamentFactory.json"), 
            middle_factory: String::from("core/artifacts/MiddleTournamentFactory.json"), 
            bottom_factory: String::from("core/artifacts/BottomTournamentFactory.json"), 
            tournament_factory: String::from("core/artifacts/TournamentFactory.json"),
        },
    };
    let arena = EthersArena::new(&arena_config, tournament_factory_address)?;
    Ok(Arc::new(arena))
}