use cryptography::hash::Hash;
use crate::machine::Machine;
use crate::constants;
use ruint::Uint;
use cryptography::merkle_tree::MerkleTree;
use jsonrpc_cartesi_machine::{JsonRpcCartesiMachineClient, MachineRuntimeConfig};
use lazy_static::lazy_static;

pub const MAX_A: Uint<256, 4> = get_mask(constants::A);
pub const MAX_B: Uint<256, 4> = get_mask(constants::B);

lazy_static! {
    static ref root_machine: JsonRpcCartesiMachineClient = JsonRpcCartesiMachineClient::new("http://localhost:8080".to_string());
}
pub struct ComputationCounter {
    stride: Uint<256, 4>,
    stride_count: Uint<256, 4>,
    current_meta_counter: Uint<256, 4>,
    current_stride_count: Uint<256, 4>,
}

impl ComputationCounter {
    pub fn new(log2_stride: u32, base_meta_counter: Uint<256, 4>, log2_stride_count: u32) -> ComputationCounter {
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

    pub async fn from_current_machine_state(machine: &JsonRpcCartesiMachineClient) -> ComputationResult {
        let hash = Hash::from_digest_bin(machine.get_root_hash().await);
        let halted = machine.read_iflags_h().await.unwrap();
        let uhalted = machine.read_uarch_halt_flag();

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

pub struct BaseMachine<'a> {
    stub: &'a JsonRpcCartesiMachineClient,
    base_cycle: u64,
}

impl<'a> BaseMachine<'a> {
    pub async fn new_root(path: &'a str) -> BaseMachine<'a> {

        root_machine.await.unwrap().load_machine(path, &MachineRuntimeConfig::default());
        let start_cycle = root_machine.await.unwrap().get_csr_address("mcycle".to_string()).await;

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        assert_eq!(root_machine.await.unwrap().root_machine.get_csr_address("uarch_cycle".to_string()), 0);

        BaseMachine {
            stub: &root_machine.await.unwrap(),
            base_cycle: start_cycle.unwrap(),
        }
    }

    pub fn create_big_machine(&self) -> BigMachine {
        let new_stub = assert!(create_stub(self.stub.fork()));
        BigMachine::new(self, new_stub)
    }

    pub fn result(&self) -> ComputationResult {
        //let machine = self.stub.get_machine();
        ComputationResult::from_current_machine_state(&self.stub).await
    }
}

pub struct BigMachine<'a> {
    base_machine: &'a BaseMachine<'a>,
    stub: &'a JsonRpcCartesiMachineClient,
    cycle: u64,
}

impl<'a> BigMachine<'a> {
    pub fn new(base_machine: &'a BaseMachine<'a>, stub: &'a JsonRpcCartesiMachineClient) -> BigMachine<'a> {
        let machine = stub.get_machine();
        assert_eq!(machine.read_mcycle(), base_machine.base_cycle);
        assert_eq!(machine.read_uarch_cycle(), 0);

        BigMachine {
            base_machine,
            stub,
            cycle: 0,
        }
    }

    pub fn advance(&mut self, cycle: u64) {
        //let machine = self.stub.get_machine();
        self.stub.run(self.base_machine.base_cycle + cycle);
        self.cycle = cycle;
    }

    pub fn get_state(&self) -> ComputationResult {
        //let machine = self.stub.get_machine();
        ComputationResult::from_current_machine_state(&self.stub).await
    }

    pub fn create_small_machine(&self) -> SmallMachine {
        let new_stub = assert!(create_stub(self.stub.fork()));
        SmallMachine::new(self, new_stub)
    }

    pub fn shutdown(&self) {
        self.stub.shutdown();
    }

    pub fn base_machine(&self) -> &BaseMachine<'a> {
        self.shutdown();
        self.base_machine
    }
}

pub struct SmallMachine<'a> {
    big_machine: &'a BigMachine<'a>,
    stub: &'a JsonRpcCartesiMachineClient,
    ucycle: u64,
}

impl<'a> SmallMachine<'a> {
    pub fn new(big_machine: &'a BigMachine<'a>, stub: &'a JsonRpcCartesiMachineClient) -> SmallMachine<'a> {
        assert_eq!(stub.read_uarch_cycle(), 0);

        SmallMachine {
            big_machine,
            stub,
            ucycle: 0,
        }
    }

    pub fn uadvance(&mut self, ucycle: u64) {
        let machine = self.stub;
        machine.run_uarch(ucycle);
        self.ucycle = ucycle;
    }

    pub fn get_state(&self) -> ComputationResult {
        let machine = self.stub.get_machine();
        ComputationResult::from_current_machine_state(&machine).await
    }

    pub fn shutdown(&self) {
        self.stub.shutdown();
    }

    pub fn shutdown_and_get_big_machine(&self) -> &BigMachine<'a> {
        self.shutdown();
        self.big_machine
    }
}

fn get_mask(k: u32) -> Uint<256, 4> {
    Uint::from((1 << k) - 1)
}

fn get_ucycle(mc: &Uint<256, 4>) -> u64 {
    assert!(mc & get_mask(constants::A) < Uint::from(std::u64::MAX));
    (mc & get_mask(constants::A)).to::<u64>()
}

fn get_cycle(mc: &Uint<256, 4>) -> u64 {
    assert!((mc >> constants::A as usize) & get_mask(constants::B) < Uint::from(std::u64::MAX));
    ((mc >> constants::A as usize) & get_mask(constants::B)).to::<u64>()
}

fn add_uintervals<F>(counter: &mut ComputationCounter, big_machine: &BigMachine, mut add_state: F)
where
    F: FnMut(Hash, Option<Uint<256, 4>>),
{
    let mut small_machine = big_machine.create_small_machine();
    let current_instruction = get_cycle(&counter.peek_next().unwrap());

    while !counter.is_empty() && current_instruction == get_cycle(&counter.peek_next().unwrap()) {
        let next_uinstruction = get_ucycle(&counter.peek_next().unwrap());
        small_machine.uadvance(next_uinstruction);
        let result = small_machine.get_state();

        if result.uhalted {
            let r = (MAX_A - Uint::from(next_uinstruction) / &counter.stride + Uint::from(1)).min(counter.len());
            add_state(result.state, Some(r.clone()));
            counter.popn(&r);
            break;
        } else {
            add_state(result.state, None);
            counter.next();
        }
    }

    small_machine.shutdown();
}

fn add_intervals<F>(counter: &mut ComputationCounter, base_machine: &BaseMachine, mut add_state: F)
where
    F: FnMut(Hash, Option<Uint<256, 4>>),
{
    let mut big_machine = base_machine.create_big_machine();

    while !counter.is_empty() {
        let next_mc = counter.peek_next().unwrap();
        let next_instruction = get_cycle(&next_mc);
        big_machine.advance(next_instruction);

        if get_ucycle(&next_mc) != 0 {
            add_uintervals(counter, &big_machine, &mut add_state);
        } else {
            let result = big_machine.get_state();

            if result.uhalted && &counter.stride & Uint::from(MAX_A) == Uint::from(0) {
                let r = (MAX_B - Uint::from(next_instruction) / &counter.stride + Uint::from(1)).min(counter.len());
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

    big_machine.shutdown();
}

fn get_leafs(log2_stride: u32, base_meta_counter: Uint<256, 4>, log2_stride_count: u32, base_machine: &BaseMachine) -> (Hash, Vec<(Hash, Option<Uint<256, 4>>)>) {
    let mut interval = ComputationCounter::new(log2_stride, base_meta_counter, log2_stride_count);
    let mut accumulator = Vec::new();

    let mut add_state = |s: Hash, r: Option<Uint<256, 4>>| {
        let r = r.unwrap_or_else(|| Uint::from(1));
        accumulator.push((s, Some(r.clone())));
        println!("{}, {:?}, {})", accumulator.len(), s, r);
    };

    add_intervals(&mut interval, base_machine, add_state);
    let inital_state = base_machine.result().state;

    (inital_state, accumulator)
}
#[tokio::main]
async fn main() {
    let base_machine = BaseMachine::new_root("program/simple-program").await;

    let mc = Uint::from(0) + (Uint::from(0) << 64);
    let (_, y) = get_leafs(5, mc, 64, &base_machine);

    for (i, v) in y.iter().enumerate() {
        println!("{}, {:?}, {:?}", i + 1, v.0, v.1);
    }
}
