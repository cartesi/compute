use std::sync::Arc;

use cartesi_compute_core::arena::{ArenaConfig, ContractArtifactsConfig, EthersArena}; 

use cartesi_compute_coordinator::server::{APIServer, APIServerConfig};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_address = "0.0.0.0:50500";
    let web3_sec_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    let web3_rpc_url = "http://localhost:8545";
    
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
    let mut arena = EthersArena::new(arena_config)?;
    arena.init().await?;
    let arena = Arc::new(arena);
    
    let server_config = APIServerConfig {
        address: String::from(api_address), 
    };
    let server = APIServer::new(arena.clone(), server_config);
    server.run().await
}