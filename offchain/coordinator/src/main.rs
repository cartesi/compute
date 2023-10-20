use std::sync::Arc;

use cartesi_compute_core::arena::{ArenaConfig, ContractArtifactsConfig, EthersArena}; 

use cartesi_compute_coordinator::server::{APIServer, APIServerConfig};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web3_rpc_url = "http://anvil:8545";
    let web3_chain_id = 31337;
    let web3_private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let api_address = "0.0.0.0:50500";
    
    let arena_config = ArenaConfig{
        web3_rpc_url: String::from(web3_rpc_url),
        web3_chain_id: web3_chain_id,
        web3_private_key: String::from(web3_private_key),
        contract_artifacts: ContractArtifactsConfig { 
            single_level_factory: String::from("core/artifacts/SingleLevelTournamentFactory.json"), 
            top_factory: String::from("core/artifacts/TopTournamentFactory.json"), 
            middle_factory: String::from("core/artifacts/MiddleTournamentFactory.json"), 
            bottom_factory: String::from("core/artifacts/BottomTournamentFactory.json"), 
            tournament_factory: String::from("core/artifacts/TournamentFactory.json"),
        },
    };
    let mut arena = EthersArena::new(arena_config)?;
    arena.init().await?;
    let arena = Arc::new(arena);
    
    let server_config = APIServerConfig {
        address: String::from(api_address), 
    };
    let server = APIServer::new(arena.clone(), server_config);
    server.run().await
}