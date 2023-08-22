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
use jsonrpc_cartesi_machine::JsonRpcCartesiMachineClient;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rstest::*;
use std::future::Future;

static INITIAL_ROOT_HASH: [u8; 32] = [
    134, 97, 187, 228, 108, 242, 89, 237, 97, 43, 127, 135, 153, 29, 132, 227, 4, 188, 239, 190,
    180, 219, 72, 86, 244, 22, 194, 24, 161, 220, 14, 237,
];

static SECOND_STEP_HASH: [u8; 32] = [
    234, 184, 50, 7, 132, 79, 19, 205, 60, 47, 240, 187, 108, 27, 112, 2, 244, 214, 86, 235, 64,
    255, 110, 182, 165, 125, 138, 144, 29, 206, 188, 236,
];

#[allow(dead_code)]
struct Context {
    cartesi_machine_server: JsonRpcCartesiMachineClient,
    server_ip: String,
    port: u32,
    container_name: String,
}

fn generate_random_name() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect()
}

fn instantiate_external_server_instance(port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("127.0.0.1:{0}", port);
    let server_address = format!("--server-address=127.0.0.1:{0}", port);

    println!(
        "Starting Cartesi jsonrpc remote machine on address {}",
        address
    );
    match std::process::Command::new("/usr/bin/jsonrpc-remote-cartesi-machine")
        .arg(server_address)
        .spawn()
    {
        Ok(child) => {}
        Err(error) => panic!(error.to_string()),
    };
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}

fn try_stop_container() {
    let result = std::process::Command::new("pkill")
        .arg("-f")
        .arg("/usr/bin/jsonrpc-remote-cartesi-machine")
        .status()
        .unwrap();
    if !result.success() {
        eprint!("Error stopping container");
    }
}

impl Context {
    pub fn get_server(&mut self) -> &mut JsonRpcCartesiMachineClient {
        &mut self.cartesi_machine_server
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Destroying container {}", &self.container_name);
        try_stop_container();
    }
}

#[allow(unused_mut)]
mod local_server {
    use super::*;

    #[fixture]
    async fn context_future() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 50051;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();

        match instantiate_external_server_instance(port) {
            Ok(_) => (),
            Err(ex) => eprint!(
                "Error instantiating cartesi machine server {}",
                ex.to_string()
            ),
        }
        println!(
            "Starting jsonrpc machine server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );

        Context {
            cartesi_machine_server: match JsonRpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
        }
    }

    #[fixture]
    async fn context_with_machine_future() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 50051;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();
        match instantiate_external_server_instance(port) {
            Ok(_) => (),
            Err(err) => eprint!(
                "Error instantiating jsonrpc cartesi machine server {}",
                err.to_string()
            ),
        }
        println!(
            "Starting jsonrpc cartesi server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );
        let mut context = Context {
            cartesi_machine_server: match JsonRpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!(
                        "Unable to create jsonrpc machine server: {}",
                        err.to_string()
                    )
                }
            },
            port,
            server_ip,
            container_name,
        };
        //Modify default configuration
        let mut default_config = match context.get_server().get_default_config().await {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to get default config: {}", err.to_string())
            }
        };
        default_config.rom = jsonrpc_cartesi_machine::RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = jsonrpc_cartesi_machine::RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        default_config.uarch = jsonrpc_cartesi_machine::UarchConfig {
            processor: Some(cartesi_jsonrpc_interfaces::index::UarchProcessorConfig {
                x: Some(vec![0; 32]),
                pc: Some(0x70000000),
                cycle: Some(0),
            }),
            ram: Some(cartesi_jsonrpc_interfaces::index::UarchRAMConfig {
                length: Some(77128),
                image_filename: Some(String::from("/opt/cartesi/share/images/uarch-ram.bin")),
            }),
        };
        default_config.htif = jsonrpc_cartesi_machine::HTIFConfig {
            console_getchar: Some(false),
            yield_manual: Some(true),
            yield_automatic: Some(false),
            fromhost: Some(0),
            tohost: Some(0),
        };
        default_config.rollup = jsonrpc_cartesi_machine::RollupConfig {
            input_metadata: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60400000,
                length: 4096,
                image_filename: "".to_string(),
                shared: false,
            }),
            notice_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60800000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            rx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60000000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            voucher_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60600000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            tx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60200000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
        };

        match context
            .get_server()
            .create_machine(
                &default_config,
                &jsonrpc_cartesi_machine::MachineRuntimeConfig::default(),
            )
            .await
        {
            Ok(_) => context,
            Err(err) => {
                panic!("Unable to instantiate cartesi machine: {}", err.to_string())
            }
        }
    }

    #[fixture]
    async fn context_with_machine_with_flash_future() -> Context {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 50051;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();
        match instantiate_external_server_instance(port) {
            Ok(_) => (),
            Err(err) => eprint!(
                "Error instantiating jsonrpc cartesi machine server {}",
                err.to_string()
            ),
        }
        println!(
            "Starting jsonrpc cartesi server: {} server_ip:{}:{} ",
            container_name, server_ip, port
        );
        let mut context = Context {
            cartesi_machine_server: match JsonRpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
        };
        //Modify default configuration
        let mut default_config = match context.get_server().get_default_config().await {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to get default config: {}", err.to_string())
            }
        };
        default_config.rom = jsonrpc_cartesi_machine::RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = jsonrpc_cartesi_machine::RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        default_config.htif = jsonrpc_cartesi_machine::HTIFConfig {
            console_getchar: Some(false),
            yield_manual: Some(true),
            yield_automatic: Some(false),
            fromhost: Some(0),
            tohost: Some(0),
        };
        default_config.rollup = jsonrpc_cartesi_machine::RollupConfig {
            input_metadata: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60400000,
                length: 4096,
                image_filename: "".to_string(),
                shared: false,
            }),
            notice_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60800000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            rx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60000000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            voucher_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60600000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            tx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60200000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
        };

        //Create flash image and add to flash configuration
        match std::fs::write("/tmp/input_root.raw", b"Root data in flash") {
            Ok(_) => (),
            Err(err) => panic!(
                "Unable to create temporary flash image: {}",
                err.to_string()
            ),
        }
        std::process::Command::new("truncate")
            .args(&["-s", "62914560", "/tmp/input_root.raw"])
            .output()
            .expect("Unable to create flash image file");
        default_config.flash_drives = vec![jsonrpc_cartesi_machine::MemoryRangeConfig {
            start: 0x80000000000000,
            image_filename: "/tmp/input_root.raw".to_string(),
            length: 0x3c00000,
            shared: false,
        }];
        //Create machine
        match context
            .get_server()
            .create_machine(
                &default_config,
                &jsonrpc_cartesi_machine::MachineRuntimeConfig::default(),
            )
            .await
        {
            Ok(_) => context,
            Err(err) => {
                panic!(
                    "Unable to instantiate jsonrpc cartesi machine: {}",
                    err.to_string()
                )
            }
        }
    }

    #[rstest]
    #[tokio::test]
    #[should_panic]
    async fn test_invalid_server_address() -> () {
        let server_ip = "127.0.0.1".to_string();
        let port: u32 = 12345;
        let uri = format!("http://{}:{}", server_ip, port);
        let container_name = generate_random_name();
        let _context = Context {
            cartesi_machine_server: match JsonRpcCartesiMachineClient::new(uri).await {
                Ok(machine) => machine,
                Err(err) => {
                    panic!("Unable to create machine server: {}", err.to_string())
                }
            },
            port,
            server_ip,
            container_name,
        };
        ()
    }

    #[rstest]
    #[tokio::test]
    async fn test_cartesi_server_instance(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        println!(
            "Sleeping in the test... context container name: {}",
            context.container_name
        );
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("End sleeping");
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_version(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let semantic_version = context.get_server().get_version().await?;
        println!("Acquired semantic version: {:?} ", semantic_version);
        assert_eq!(
            semantic_version,
            jsonrpc_cartesi_machine::SemanticVersion {
                major: 0,
                minor: 1,
                patch: 1,
                pre_release: "".to_string(),
                build: "".to_string()
            }
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_default_config(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let default_config = context.get_server().get_default_config().await?;
        println!("Acquired default config {:?}", default_config);
        assert_eq!(default_config.processor.pc, 4096);
        assert_eq!(default_config.processor.mvendorid, 7161130726739634464);
        assert_eq!(default_config.processor.marchid, 0xf);
        assert_eq!(default_config.ram.length, 0);
        assert_eq!(default_config.rom.image_filename, "");
        assert_eq!(default_config.flash_drives.len(), 0);
        assert_eq!(default_config.htif.fromhost, Some(0));
        assert_eq!(default_config.htif.tohost, Some(0));
        assert_eq!(default_config.clint.mtimecmp, Some(0));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_machine_create(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let mut default_config = context.get_server().get_default_config().await?;
        default_config.rom = jsonrpc_cartesi_machine::RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = jsonrpc_cartesi_machine::RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        default_config.htif = jsonrpc_cartesi_machine::HTIFConfig {
            console_getchar: Some(false),
            yield_manual: Some(true),
            yield_automatic: Some(false),
            fromhost: Some(0),
            tohost: Some(0),
        };
        default_config.rollup = jsonrpc_cartesi_machine::RollupConfig {
            input_metadata: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60400000,
                length: 4096,
                image_filename: "".to_string(),
                shared: false,
            }),
            notice_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60800000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            rx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60000000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            voucher_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60600000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            tx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60200000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
        };

        context
            .get_server()
            .create_machine(
                &default_config,
                &jsonrpc_cartesi_machine::MachineRuntimeConfig::default(),
            )
            .await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_machine_create_already_created(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let mut default_config = context.get_server().get_default_config().await?;
        default_config.rom = jsonrpc_cartesi_machine::RomConfig {
            bootargs: default_config.rom.bootargs,
            image_filename: String::from("/opt/cartesi/share/images/rom.bin"),
        };
        default_config.ram = jsonrpc_cartesi_machine::RamConfig {
            length: 1 << 20,
            image_filename: String::new(),
        };
        default_config.uarch = jsonrpc_cartesi_machine::UarchConfig {
            processor: Some(cartesi_jsonrpc_interfaces::index::UarchProcessorConfig {
                x: Some(vec![0; 32]),
                pc: Some(0x70000000),
                cycle: Some(0),
            }),
            ram: Some(cartesi_jsonrpc_interfaces::index::UarchRAMConfig {
                length: Some(77128),
                image_filename: Some(String::from("/opt/cartesi/share/images/uarch-ram.bin")),
            }),
        };
        default_config.htif = jsonrpc_cartesi_machine::HTIFConfig {
            console_getchar: Some(false),
            yield_manual: Some(true),
            yield_automatic: Some(false),
            fromhost: Some(0),
            tohost: Some(0),
        };
        default_config.rollup = jsonrpc_cartesi_machine::RollupConfig {
            input_metadata: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60400000,
                length: 4096,
                image_filename: "".to_string(),
                shared: false,
            }),
            notice_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60800000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            rx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60000000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            voucher_hashes: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60600000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
            tx_buffer: Some(jsonrpc_cartesi_machine::MemoryRangeConfig {
                start: 0x60200000,
                length: 2 << 20,
                image_filename: "".to_string(),
                shared: false,
            }),
        };
        let ret = context
            .get_server()
            .create_machine(
                &default_config,
                &jsonrpc_cartesi_machine::MachineRuntimeConfig::default(),
            )
            .await;
        match ret {
            Ok(_) => panic!("Creating existing machine should fail"),
            Err(err) => assert_eq!(
                err.to_string(),
                "ErrorObject { code: InvalidRequest, message: \"machine exists\", data: None }"
            ),
        }
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_run(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let run_response = context.get_server().run(1000).await?;
        assert_eq!(run_response, "reached_target_mcycle");

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_store(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context
            .get_server()
            .store(&format!("/tmp/cartesi_{}", generate_random_name()))
            .await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_store_nomachine(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let ret = context.get_server().store("/tmp/cartesi_store").await;
        assert!(ret.is_err());
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_destroy(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().destroy().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_fork(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let address = context.get_server().fork().await?;
        let uri = format!("http://{}", address);
        JsonRpcCartesiMachineClient::new(uri).await.unwrap();
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_step(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let log = context
            .get_server()
            .step(
                &jsonrpc_cartesi_machine::AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        //println!("Acquired log for step: {:?} ", log);
        assert!(log.accesses.len() > 0);
        assert!(log.accesses[0].r#type == jsonrpc_cartesi_machine::AccessType::Read);
        assert!(log.brackets.len() > 0);
        assert!(log.log_type.proofs == true && log.log_type.annotations == true);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_shutdown(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().shutdown().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_double_shutdown(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().shutdown().await?;
        let ret = context.get_server().shutdown().await;
        assert!(ret.is_err());
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_memory(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().read_memory(0x1000, 16).await?;
        assert_eq!(
            ret,
            vec![151, 2, 0, 0, 147, 130, 162, 4, 115, 144, 82, 48, 65, 101, 189, 101]
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_write_memory(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context
            .get_server()
            .write_memory(
                0x8000000F,
                base64::encode([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
            )
            .await?;
        let ret = context.get_server().read_memory(0x8000000F, 12).await?;
        assert_eq!(ret, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_word(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().read_word(0x100).await?;
        assert_eq!(ret, 0);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_root_hash(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().get_root_hash().await?;
        assert_eq!(ret, INITIAL_ROOT_HASH);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_root_hash_after_step(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().get_root_hash().await?;
        assert_eq!(ret, INITIAL_ROOT_HASH);
        let _log = context
            .get_server()
            .step(
                &jsonrpc_cartesi_machine::AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        let ret = context.get_server().get_root_hash().await?;
        assert_eq!(ret, SECOND_STEP_HASH);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_proof(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let proof = context.get_server().get_proof(0x0, 10).await?;
        assert_eq!(proof.log2_target_size, 10);
        let mut target_hash_string = proof.target_hash.clone();
        if target_hash_string.ends_with('\n') {
            target_hash_string.pop();
        }
        assert_eq!(
            base64::decode(target_hash_string).unwrap(),
            [
                26, 138, 138, 99, 157, 21, 172, 88, 187, 128, 223, 156, 164, 60, 123, 42, 42, 125,
                169, 162, 174, 170, 76, 51, 148, 162, 125, 172, 221, 244, 71, 27
            ]
        );
        assert_eq!(proof.sibling_hashes.len(), 54);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_replace_memory_range(
        context_with_machine_with_flash_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_with_flash_future.await;
        std::fs::write("/tmp/input.raw", b"test data 1234567890")?;
        std::process::Command::new("truncate")
            .args(&["-s", "62914560", "/tmp/input.raw"])
            .output()
            .expect("Unable to create flash image file");

        let memory_range_config = jsonrpc_cartesi_machine::MemoryRangeConfig {
            start: 0x80000000000000,
            image_filename: "/tmp/input.raw".to_string(),
            length: 0x3c00000,
            shared: true,
        };
        context
            .get_server()
            .replace_memory_range(cartesi_jsonrpc_interfaces::index::MemoryRangeConfig::from(
                &memory_range_config,
            ))
            .await?;
        let ret = context
            .get_server()
            .read_memory(0x80000000000000, 12)
            .await?;
        assert_eq!(
            ret,
            vec![116, 101, 115, 116, 32, 100, 97, 116, 97, 32, 49, 50]
        );
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_x_address(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let x_address = context.get_server().get_x_address(2).await?;
        assert_eq!(x_address, 0x10);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_write_x(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let x_value = context.get_server().read_x(2).await?;
        assert_eq!(x_value, 0x0);
        context.get_server().write_x(2, 0x1234).await?;
        let x_value = context.get_server().read_x(2).await?;
        assert_eq!(x_value, 0x1234);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_reset_i_flags_y(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().reset_iflags_y().await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_csr_address(
        context_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_future.await;
        let address = context
            .get_server()
            .get_csr_address("pc".to_string())
            .await?;
        println!("Got address: {}", address);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_read_write_csr(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let x_value = context
            .get_server()
            .read_csr("sscratch".to_string())
            .await?;
        assert_eq!(x_value, 0x0);
        context
            .get_server()
            .write_csr("sscratch".to_string(), 0x12345)
            .await?;
        let x_value = context
            .get_server()
            .read_csr("sscratch".to_string())
            .await?;
        assert_eq!(x_value, 0x12345);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_initial_config(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let initial_config = context.get_server().get_initial_config().await?;
        println!("Acquired initial config {:?}", initial_config);
        assert_eq!(initial_config.processor.pc, 4096);
        assert_eq!(initial_config.processor.mvendorid, 7161130726739634464);
        assert_eq!(initial_config.processor.marchid, 0xf);
        assert_eq!(initial_config.ram.length, 1048576);
        assert_eq!(
            initial_config.rom.image_filename,
            "/opt/cartesi/share/images/rom.bin"
        );
        assert_eq!(initial_config.flash_drives.len(), 0);
        assert_eq!(initial_config.htif.fromhost, Some(0));
        assert_eq!(initial_config.htif.tohost, Some(0));
        assert_eq!(initial_config.clint.mtimecmp, Some(0));
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_merkle_tree(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().verify_merkle_tree().await?;
        assert!(ret);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_dirty_page_maps(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let ret = context.get_server().verify_dirty_page_maps().await?;
        assert!(ret);
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_dump_pmas(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        context.get_server().dump_pmas().await?;
        std::thread::sleep(std::time::Duration::from_secs(3));
        std::process::Command::new("rm")
            .args(&[
                "0000000000000000--0000000000001000.bin",
                "0000000000001000--000000000000f000.bin",
                "0000000002000000--00000000000c0000.bin",
                "0000000040008000--0000000000001000.bin",
                "0000000080000000--0000000000100000.bin",
            ])
            .status()
            .expect("Failed to cleanup dump pmas test");
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_access_log(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let log = context
            .get_server()
            .step(
                &jsonrpc_cartesi_machine::AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        context
            .get_server()
            .verify_access_log(
                &log,
                &jsonrpc_cartesi_machine::MachineRuntimeConfig::default(),
                false,
            )
            .await?;
        Ok(())
    }

    #[rstest]
    #[tokio::test]
    async fn test_verify_state_transition(
        context_with_machine_future: impl Future<Output = Context>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = context_with_machine_future.await;
        let root_hash_before = context.get_server().get_root_hash().await?;
        let log = context
            .get_server()
            .step(
                &jsonrpc_cartesi_machine::AccessLogType {
                    annotations: true,
                    proofs: true,
                },
                false,
            )
            .await?;
        let root_hash_after = context.get_server().get_root_hash().await?;
        context
            .get_server()
            .verify_state_transition(
                root_hash_before.clone(),
                &log,
                root_hash_after.clone(),
                false,
                &jsonrpc_cartesi_machine::MachineRuntimeConfig::default(),
            )
            .await?;
        Ok(())
    }
}