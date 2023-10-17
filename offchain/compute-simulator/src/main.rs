use std::error::Error;

use cartesi_machine_json_rpc::client::{
    JsonRpcCartesiMachineClient,
    MachineRuntimeConfig,
};

use cartesi_compute_server::{
    grpc::{
        ComputeClient,
        StartDisputeRequest,
    },
    merkle::Hash,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simple_linux_program = "/data/programs/simple-linux-program";
    let simple_program = "/data/programs/simple-program";

    start_dipuste(simple_linux_program).await?;

    Ok(())
}

pub async fn start_dipuste(snapshot_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let machine_rpc_host= "http://machine";
    let machine_rpc_port = 5002;
    let server_rpc_address = "127.0.01:50500";
    
    let machine_rpc_url = format!("{}:{}", machine_rpc_host, machine_rpc_port); 
    let mut machine_client = JsonRpcCartesiMachineClient::new(machine_rpc_url).await?;
    
    let fork_rpc_url = machine_client.fork().await?;
    let fork_rpc_port = fork_rpc_url.split(":").last().unwrap();
    let fork_rpc_url = format!("{}:{}", machine_rpc_host, fork_rpc_port);

    let mut machine_fork_client = JsonRpcCartesiMachineClient::new(fork_rpc_url).await?;
    machine_fork_client.load_machine(snapshot_path, &MachineRuntimeConfig::default()).await?;

    panic!("booom");

    // !!!
    /*
    let initial_hash = machine_fork_client.get_root_hash().await?;
    let mut client = ComputeClient::connect(server_rpc_address).await?;
    client.start_dispute(StartDisputeRequest{
        initial_hash: initial_hash.into(),
        snapshot_path: String::from(snapshot_path),
    }).await?;
    */
    
    Ok(())
}