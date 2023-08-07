use crate::constants;
use crate::machine::Machine;
use cryptography::hash::Hash;
use cryptography::merkle_tree::MerkleTree;
use jsonrpc_cartesi_machine::{JsonRpcCartesiMachineClient, MachineRuntimeConfig};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use ruint::Uint;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

static ROOT_MACHINE: OnceCell<JsonRpcCartesiMachineClient> = OnceCell::new();

async fn initialize_machine() -> JsonRpcCartesiMachineClient {
    let url = "http://127.0.0.1:50051".to_string();
    JsonRpcCartesiMachineClient::new(url).await.unwrap()
}

fn initialize_root_machine() -> Arc<Mutex<JsonRpcCartesiMachineClient>> {
    let rt = Runtime::new().unwrap();
    let machine = tokio::task::block_in_place(|| rt.block_on(initialize_machine()));
    Arc::new(Mutex::new(machine))
}
pub struct ComputationCounter {
    stride: Uint<256, 4>,
    stride_count: Uint<256, 4>,
    current_meta_counter: Uint<256, 4>,
    current_stride_count: Uint<256, 4>,
}

impl ComputationCounter {
    pub fn new(
        log2_stride: u32,
        base_meta_counter: Uint<256, 4>,
        log2_stride_count: u32,
    ) -> ComputationCounter {
        let stride = Uint::from(1 << log2_stride);
        let stride_count = Uint::from(1 << log2_stride_count);

        ComputationCounter {
            stride,
            stride_count,
            current_meta_counter: base_meta_counter,
            current_stride_count: Uint::from(0),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stride_count == self.current_stride_count
    }

    pub fn len(&self) -> Uint<256, 4> {
        &self.stride_count - &self.current_stride_count
    }

    pub fn current(&self) -> &Uint<256, 4> {
        &self.current_meta_counter
    }

    pub fn peek_next(&self) -> Option<Uint<256, 4>> {
        if self.is_empty() {
            None
        } else {
            Some(&self.current_meta_counter + &self.stride)
        }
    }

    pub fn next(&mut self) -> Option<Uint<256, 4>> {
        if let Some(new_meta_counter) = self.peek_next() {
            self.current_stride_count += Uint::from(1);
            self.current_meta_counter = new_meta_counter.clone();
            Some(new_meta_counter)
        } else {
            None
        }
    }

    pub fn popn(&mut self, n: &Uint<256, 4>) {
        assert!(n < &(self.stride_count - &self.current_stride_count));
        let new_meta_counter = &self.current_meta_counter + &self.stride * n;
        self.current_stride_count += n;
        self.current_meta_counter = new_meta_counter;
    }
}

pub struct ComputationResult {
    state: Hash,
    halted: bool,
    uhalted: bool,
}

impl ComputationResult {
    pub fn new(state: Hash, halted: bool, uhalted: bool) -> ComputationResult {
        ComputationResult {
            state,
            halted,
            uhalted,
        }
    }

    pub async fn from_current_machine_state(
        machine: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    ) -> ComputationResult {
        let hash = Hash::from_digest_bin(&machine.lock().unwrap().get_root_hash().await.unwrap());
        let halted = machine.lock().unwrap().read_iflags_h().await.unwrap();
        let uhalted = machine
            .lock()
            .unwrap()
            .read_uarch_halt_flag()
            .await
            .unwrap();

        ComputationResult::new(hash, halted, uhalted)
    }
}

impl std::fmt::Display for ComputationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{state = {:?}, halted = {}, uhalted = {}}}",
            self.state, self.halted, self.uhalted
        )
    }
}
#[derive(Clone)]
pub struct BaseMachine {
    stub: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    base_cycle: u64,
}

impl BaseMachine {
    pub async fn new_root(path: &str) -> BaseMachine {
          //let machine_future = root_machine.lock().unwrap();
        //let future_machine: &dyn std::future::Future<Output = Result<JsonRpcCartesiMachineClient, jsonrpsee::core::error::Error>> = &*machine_future;

        // Now, you need to await the Future to get the actual JsonRpcCartesiMachineClient
        // Note: This requires being within an async context (async function or block)
        //let result: Result<JsonRpcCartesiMachineClient, jsonrpsee::core::error::Error> = future_machine.await;

        /*ROOT_MACHINE
            .get()
            .unwrap()
            .load_machine(path, &MachineRuntimeConfig::default())
            .await
            .unwrap();*/
        let mut default_config = ROOT_MACHINE.get()
        .unwrap().get_default_config().await.unwrap();

        default_config.rom = jsonrpc_cartesi_machine::RomConfig {
            bootargs: String::from("console=hvc0 -- ls /bin"),
            image_filename: String::from("program/bins/bootstrap.bin"),
        };
        default_config.ram = jsonrpc_cartesi_machine::RamConfig {
            length: 0x4000000,
            image_filename: String::from("program/bins/rv64ui-p-addi.bin"),
        };
    
        default_config.uarch = jsonrpc_cartesi_machine::UarchConfig {
            processor: Some(cartesi_jsonrpc_interfaces::index::UarchProcessorConfig {
                x: Some(vec![0; 32]),
                pc: Some(0x70000000),
                cycle: Some(0),
            }),
            ram: Some(cartesi_jsonrpc_interfaces::index::UarchRAMConfig {
                length: Some(0x1000000),
                image_filename: Some(String::from("share/images/uarch-ram.bin")),
            }),
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
        ROOT_MACHINE
            .get()
            .unwrap()
            .create_machine(&default_config, &MachineRuntimeConfig::default())
            .await
            .unwrap();

        let start_cycle = ROOT_MACHINE
            .get()
            .unwrap().clone()
            .get_csr_address("mcycle".to_string())
            .await;

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        /*assert_eq!(
            ROOT_MACHINE
                .get()
                .unwrap().clone()
                .get_csr_address("uarch_cycle".to_string())
                .await
                .unwrap(),
            0
        );*/

        BaseMachine {
            stub: Arc::new(Mutex::new(ROOT_MACHINE
                .get().unwrap().clone())),
            base_cycle: start_cycle.unwrap(),
        }
    }

    pub async fn create_big_machine(self) -> Arc<Mutex<BigMachine>> {
        let new_stub = Arc::new(Mutex::new(
            JsonRpcCartesiMachineClient::new(self.stub.lock().unwrap().fork().await.unwrap())
                .await
                .unwrap(),
        ));
        Arc::new(Mutex::new(BigMachine::new(self, new_stub).await))
    }

    pub async fn result(&self) -> ComputationResult {
        //let machine = self.stub.get_machine();
        ComputationResult::from_current_machine_state(Arc::clone(&self.stub)).await
    }
}
#[derive(Clone)]
pub struct BigMachine {
    base_machine: BaseMachine,
    stub: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    cycle: u64,
}

impl BigMachine {
    pub async fn new(
        base_machine: BaseMachine,
        stub: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    ) -> BigMachine {
        assert_eq!(
            stub.lock()
                .unwrap()
                .get_csr_address("mcycle".to_string())
                .await
                .unwrap(),
            base_machine.base_cycle
        );
        assert_eq!(
            stub.lock()
                .unwrap()
                .get_csr_address("uarch_cycle".to_string())
                .await
                .unwrap(),
            0
        );

        BigMachine {
            base_machine,
            stub,
            cycle: 0,
        }
    }

    pub fn advance(&mut self, cycle: u64) {
        //let machine = self.stub.get_machine();
        self.stub
            .lock()
            .unwrap()
            .run(self.base_machine.base_cycle + cycle);
        self.cycle = cycle;
    }

    pub async fn get_state(&mut self) -> ComputationResult {
        ComputationResult::from_current_machine_state(Arc::clone(&self.stub)).await
    }

    pub async fn create_small_machine(self) -> Arc<Mutex<SmallMachine>> {
        let new_stub = Arc::new(Mutex::new(
            JsonRpcCartesiMachineClient::new(self.stub.lock().unwrap().fork().await.unwrap())
                .await
                .unwrap(),
        ));
        Arc::new(Mutex::new(
            SmallMachine::new(Arc::new(Mutex::new(self)), new_stub).await,
        ))
    }

    pub fn shutdown(&self) {
        self.stub.lock().unwrap().shutdown();
    }

    pub fn base_machine(&self) -> BaseMachine {
        self.shutdown();
        self.base_machine.clone()
    }
}
#[derive(Clone)]
pub struct SmallMachine {
    big_machine: Arc<Mutex<BigMachine>>,
    stub: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    ucycle: u64,
}

impl SmallMachine {
    pub async fn new(
        big_machine: Arc<Mutex<BigMachine>>,
        stub: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    ) -> SmallMachine {
        assert_eq!(
            stub.lock()
                .unwrap()
                .get_csr_address("uarch_cycle".to_string())
                .await
                .unwrap(),
            0
        );

        SmallMachine {
            big_machine,
            stub,
            ucycle: 0,
        }
    }

    pub fn uadvance(&mut self, ucycle: u64) {
        let machine = self.stub.lock().unwrap();
        machine.run_uarch(ucycle);
        self.ucycle = ucycle;
    }

    pub async fn get_state(&self) -> ComputationResult {
        //let machine = self.stub.get_machine();
        ComputationResult::from_current_machine_state(Arc::clone(&self.stub)).await
    }

    pub fn shutdown(&self) {
        self.stub.lock().unwrap().shutdown();
    }

    pub fn shutdown_and_get_big_machine(&self) -> Arc<Mutex<BigMachine>> {
        self.shutdown();
        Arc::clone(&self.big_machine)
    }
}

fn get_mask(k: u32) -> Uint<256, 4> {
    Uint::from((1 << k) - 1)
}

fn get_ucycle(mc: &Uint<256, 4>) -> u64 {
    assert!(mc & get_mask(constants::LOG2_EMULATOR_SPAN) < Uint::from(std::u64::MAX));
    (mc & get_mask(constants::LOG2_EMULATOR_SPAN)).to::<u64>()
}

fn get_cycle(mc: &Uint<256, 4>) -> u64 {
    assert!((mc >> constants::LOG2_EMULATOR_SPAN as usize) & get_mask(constants::LOG2_EMULATOR_SPAN) < Uint::from(std::u64::MAX));
    ((mc >> constants::LOG2_EMULATOR_SPAN as usize) & get_mask(constants::LOG2_EMULATOR_SPAN)).to::<u64>()
}

async fn add_uintervals<F>(
    counter: &mut ComputationCounter,
    big_machine: Arc<Mutex<BigMachine>>,
    mut add_state: F,
) where
    F: FnMut(Hash, Option<Uint<256, 4>>),
{
    let mut small_machine = big_machine
        .lock()
        .unwrap()
        .clone()
        .create_small_machine()
        .await;
    let current_instruction = get_cycle(&counter.peek_next().unwrap());

    while !counter.is_empty() && current_instruction == get_cycle(&counter.peek_next().unwrap()) {
        let next_uinstruction = get_ucycle(&counter.peek_next().unwrap());
        small_machine.lock().unwrap().uadvance(next_uinstruction);
        let result = small_machine.lock().unwrap().get_state().await;

        if result.uhalted {
            let r = (get_mask(constants::LOG2_EMULATOR_SPAN) - Uint::from(next_uinstruction) / &counter.stride
                + Uint::from(1))
            .min(counter.len());
            add_state(result.state, Some(r.clone()));
            counter.popn(&r);
            break;
        } else {
            add_state(result.state, None);
            counter.next();
        }
    }

    small_machine.lock().unwrap().shutdown();
}

async fn add_intervals<F>(
    counter: &mut ComputationCounter,
    base_machine: BaseMachine,
    mut add_state: F,
) where
    F: FnMut(Hash, Option<Uint<256, 4>>),
{
    let mut big_machine = base_machine.clone().create_big_machine().await;

    while !counter.is_empty() {
        let next_mc = counter.peek_next().unwrap();
        let next_instruction = get_cycle(&next_mc);
        big_machine.lock().unwrap().advance(next_instruction);

        if get_ucycle(&next_mc) != 0 {
            add_uintervals(counter, Arc::clone(&big_machine), &mut add_state);
        } else {
            let result = big_machine.lock().unwrap().get_state().await;

            if result.uhalted
                && &counter.stride & Uint::from(get_mask(constants::LOG2_EMULATOR_SPAN)) == Uint::from(0)
            {
                let r = (get_mask(constants::LOG2_EMULATOR_SPAN) - Uint::from(next_instruction) / &counter.stride
                    + Uint::from(1))
                .min(counter.len());
                add_state(result.state, Some(r.clone()));
                counter.popn(&r);
                assert!(counter.is_empty());
                break;
            } else {
                add_state(result.state, None);
                counter.next();
            }
        }
    }

    big_machine.lock().unwrap().shutdown();
}

async fn get_leafs(
    log2_stride: u32,
    base_meta_counter: Uint<256, 4>,
    log2_stride_count: u32,
    base_machine: BaseMachine,
) -> (Hash, Vec<(Hash, Option<Uint<256, 4>>)>) {
    let mut interval = ComputationCounter::new(log2_stride, base_meta_counter, log2_stride_count);
    let mut accumulator = Vec::new();

    let mut add_state = |s: Hash, r: Option<Uint<256, 4>>| {
        let r = r.unwrap_or_else(|| Uint::from(1));
        accumulator.push((s.clone(), Some(r.clone())));
        println!("{}, {:?}, {})", accumulator.len(), s, r);
    };

    add_intervals(&mut interval, base_machine.clone(), add_state);
    let inital_state = base_machine.result().await.state;

    (inital_state, accumulator)
}
pub async fn test_execution() -> Vec<(cryptography::hash::Hash, Option<Uint<256, 4>>)>{
    ROOT_MACHINE.set(initialize_machine().await);
    let base_machine = BaseMachine::new_root("program/simple-program").await;

    let mc = Uint::from(0) + (Uint::from(0) << 64);
    let (_, y) = get_leafs(5, mc, 64, base_machine).await;

    /*for (i, v) in y.iter().enumerate() {
        println!("{}, {:?}, {:?}", i + 1, v.0, v.1);
    }*/
    y
}
