// Copyright (C) 2021 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate jsonrpc_cartesi_machine;

use jsonrpc_cartesi_machine::{
    AccessLogType, JsonRpcCartesiMachineClient, MachineRuntimeConfig, MemoryRangeConfig, RamConfig,
    RollupConfig, RomConfig, UarchConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Starting jsonrpc cartesi test client for address {}", args[1]);

    let mut jsonrpc_machine = JsonRpcCartesiMachineClient::new(args[1].clone()).await?;
  
    let mut default_config = jsonrpc_machine.get_default_config().await?;
    println!(
        "I have got default jsonrpc cartesi machine config: {:#?}",
        default_config
    );

    let x_addr = jsonrpc_machine.get_x_address(3).await?;
    println!("I got x address of register 3: {}", x_addr);

    let csr_addr = jsonrpc_machine.get_csr_address("mcycle".to_string()).await?;
    println!("I got csr address of mcycle reg: {}", csr_addr);

    let semantic_version = jsonrpc_machine.get_version().await?;
    println!("I got dhd  address of reg index 3: {:#?}", semantic_version);

    default_config.rom = RomConfig {
        bootargs: default_config.rom.bootargs,
        image_filename: String::from("share/images/rom.bin"),
    };
    default_config.ram = RamConfig {
        length: 1 << 20,
        image_filename: String::new(),
    };

    default_config.uarch = UarchConfig {
        processor: Some(cartesi_jsonrpc_interfaces::index::UarchProcessorConfig {
            x: Some(vec![0; 32]),
            pc: Some(0x70000000),
            cycle: Some(0),
        }),
        ram: Some(cartesi_jsonrpc_interfaces::index::UarchRAMConfig {
            length: Some(77128),
            image_filename: Some(String::from("share/images/uarch-ram.bin")),
        }),
    };

    default_config.rollup = RollupConfig {
        input_metadata: Some(MemoryRangeConfig {
            start: 0x60400000,
            length: 4096,
            image_filename: "".to_string(),
            shared: false,
        }),
        notice_hashes: Some(MemoryRangeConfig {
            start: 0x60800000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
        rx_buffer: Some(MemoryRangeConfig {
            start: 0x60000000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
        voucher_hashes: Some(MemoryRangeConfig {
            start: 0x60600000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
        tx_buffer: Some(MemoryRangeConfig {
            start: 0x60200000,
            length: 2 << 20,
            image_filename: "".to_string(),
            shared: false,
        }),
    };

    jsonrpc_machine
        .create_machine(&default_config, &MachineRuntimeConfig::default())
        .await?;
    //println!("I got dhd  address of reg index 3: {:#?}", semantic_version);

    let hash = jsonrpc_machine.get_root_hash().await?;
    println!("Root hash step 0 {:?}", hash);

    let access_log = jsonrpc_machine
        .step(
            &AccessLogType {
                annotations: true,
                proofs: true,
            },
            true,
        )
        .await?;
    println!(
        "Step performed, number of accesses: {} ",
        access_log.accesses.len()
    );

    let hash = jsonrpc_machine.get_root_hash().await?;
    println!("Root hash step 1 {:?}", hash);

    let run_info = jsonrpc_machine.run(100).await?;
    println!(
        "Run info: {}",
        run_info
    );

    jsonrpc_machine.destroy().await?;
    println!("Machine destroyed");

    jsonrpc_machine.shutdown().await?;
    println!("Server shut down");

    Ok(())
}
