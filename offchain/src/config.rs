use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub web3_http_url: String,
    pub wallet_private_key: String,
    pub player_react_period: Duration,
}