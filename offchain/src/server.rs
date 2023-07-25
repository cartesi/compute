use std::time::Duration;

use tonic::transport::Server;

use cartesi_compute::grpc::ComputeServer;
use cartesi_compute::manager::ComputeManager;

use cartesi_compute::config::Config;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config{
        web3_http_url: String::from("http://localhost:8545"),
        wallet_private_key: String::from("dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"),
        player_react_period: Duration::from_secs(5),
    };
    let compute = ComputeManager::new(&config);

    println!("Starting gRPC Server...");

    let server_addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(ComputeServer::new(compute))
        .serve(server_addr)
        .await?;

    Ok(())
}