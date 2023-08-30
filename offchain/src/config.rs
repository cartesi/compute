use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ArenaConfig {
    pub web3_http_url: String,
    pub private_key: String,
    pub contract_artifacts: ContractArtifactsConfig,
}

#[derive(Debug, Clone)]
pub struct ContractArtifactsConfig {
    pub single_level_factory: String,
    pub top_factory: String,
    pub middle_factory: String,
    pub bottom_factory: String,
    pub tournament_factory: String,
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub player_react_period: Duration, 
}

#[derive(Debug, Clone)]
pub struct APIServerConfig {
    pub address: String,
}
