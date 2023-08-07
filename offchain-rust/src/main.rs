use std::process::Command;

use computation::{commitment, machine_test};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let output = Command::new("sh")
        .arg("program/gen_machine_simple.sh")
        .output()
        .expect("Failed to execute program/gen_machine_simple.sh");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Script output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Script execution failed: {}", stderr);
    }    
    instantiate_external_server_instance(50051);

    commitment::commitment_execution().await;

    // os.execute "jsonrpc-remote-cartesi-machine --server-address=localhost:8080 &"
    // os.execute "sleep 2"
    // require "cryptography.merkle_builder"

    // require "computation.machine_test"

    // local utils = require "utils"
    // local cartesi = {}
    // cartesi.rpc = require"cartesi.grpc"

    // local remote = cartesi.rpc.stub("localhost:8080", "localhost:8081")
    // local v = assert(remote.get_version())
    // print(string.format("Connected: remote version is %d.%d.%d\n", v.major, v.minor, v.patch))

    // local machine = remote.machine("program/simple-program")
    // print("cycles", machine:read_mcycle(), machine:read_uarch_cycle())
    // machine:snapshot()
    // machine:snapshot()

    // print(utils.hex_from_bin(machine:get_root_hash()))
    // machine:run(1000)
    // print(machine:read_iflags_H(), utils.hex_from_bin(machine:get_root_hash()))
    // machine:rollback()

    // print(utils.hex_from_bin(machine:get_root_hash()))
    // machine:run(1000)
    // print(machine:read_iflags_H(), utils.hex_from_bin(machine:get_root_hash()))
    // machine:rollback()

    // print(utils.hex_from_bin(machine:get_root_hash()))
    // machine:run(1000)
    // print(machine:read_iflags_H(), utils.hex_from_bin(machine:get_root_hash()))

    // machine:read_mcycle()

    println!("Good-bye, world!");
}

fn instantiate_external_server_instance(port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("127.0.0.1:{0}", port);
    println!(
        "Starting Cartesi jsonrpc remote machine on address {}",
        address
    );
    std::process::Command::new("/opt/cartesi/bin/jsonrpc-remote-cartesi-machine")
        .arg(&address)
        .spawn()
        .expect("Unable to launch jsonrpc cartesi machine server");
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}
