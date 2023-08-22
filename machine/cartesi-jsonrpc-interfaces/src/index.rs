extern crate derive_builder;
extern crate serde;
extern crate serde_json;

use core::future::Future;
use core::pin::Pin;
use derive_builder::Builder;
use jsonrpsee::core::Error;
use serde::{Deserialize, Serialize};

pub type UnsignedInteger = u64;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct CLINTConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtimecmp: Option<UnsignedInteger>,
}

pub type StringDoaGddGA = String;
pub type BooleanVyG3AETh = bool;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct HTIFRuntimeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_console_putchar: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct MemoryRangeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<BooleanVyG3AETh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<UnsignedInteger>,
}
pub type FlashDriveConfigs = Vec<MemoryRangeConfig>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct HTIFConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_getchar: Option<BooleanVyG3AETh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromhost: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tohost: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_automatic: Option<BooleanVyG3AETh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_manual: Option<BooleanVyG3AETh>,
}
pub type FRegConfig = Vec<UnsignedInteger>;
pub type XRegConfig = Vec<UnsignedInteger>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ProcessorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f: Option<FRegConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcsr: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icycleinstret: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iflags: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ilrsc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marchid: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcause: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcounteren: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcycle: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medeleg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menvcfg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mepc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mideleg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mie: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimpid: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mip: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misa: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mscratch: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstatus: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtval: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtvec: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mvendorid: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satp: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scause: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scounteren: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senvcfg: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sscratch: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stval: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stvec: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<XRegConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct RAMConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
    pub length: UnsignedInteger,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct RollupConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_metadata: Option<MemoryRangeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_hashes: Option<MemoryRangeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_buffer: Option<MemoryRangeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_buffer: Option<MemoryRangeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_hashes: Option<MemoryRangeConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ROMConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootargs: Option<StringDoaGddGA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct TLBConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct UarchProcessorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<UnsignedInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<XRegConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct UarchRAMConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filename: Option<StringDoaGddGA>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<UnsignedInteger>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct UarchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<UarchProcessorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<UarchRAMConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ConcurrencyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_merkle_tree: Option<UnsignedInteger>,
}
pub type Base64Hash = String;
pub type Base64HashArray = Vec<Base64Hash>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Proof {
    #[serde(rename = "log2_root_size")]
    pub log_2_root_size: UnsignedInteger,
    #[serde(rename = "log2_target_size")]
    pub log_2_target_size: UnsignedInteger,
    pub root_hash: Base64Hash,
    pub sibling_hashes: Base64HashArray,
    pub target_address: UnsignedInteger,
    pub target_hash: Base64Hash,
}
pub type Base64String = String;
pub type AccessType = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Access {
    pub address: UnsignedInteger,
    #[serde(rename = "log2_size")]
    pub log_2_size: UnsignedInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Proof>,
    pub read: Base64String,
    #[serde(rename = "type")]
    pub r#type: AccessType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written: Option<Base64String>,
}
pub type AccessArray = Vec<Access>;
pub type BracketType = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Bracket {
    pub text: StringDoaGddGA,
    #[serde(rename = "type")]
    pub r#type: BracketType,
    pub r#where: UnsignedInteger,
}
pub type BracketArray = Vec<Bracket>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct AccessLogType {
    pub has_annotations: BooleanVyG3AETh,
    pub has_proofs: BooleanVyG3AETh,
}
pub type NoteArray = Vec<StringDoaGddGA>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct MachineConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clint: Option<CLINTConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_drive: Option<FlashDriveConfigs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htif: Option<HTIFConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<ProcessorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<RAMConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollup: Option<RollupConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<ROMConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlb: Option<TLBConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uarch: Option<UarchConfig>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct MachineRuntimeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<ConcurrencyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htif: Option<HTIFRuntimeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_root_hash_check: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_version_check: Option<bool>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct AccessLog {
    pub accesses: AccessArray,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brackets: Option<BracketArray>,
    pub log_type: AccessLogType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteArray>,
}
pub type CSR = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct SemanticVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<StringDoaGddGA>,
    pub major: UnsignedInteger,
    pub minor: UnsignedInteger,
    pub patch: UnsignedInteger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_release: Option<StringDoaGddGA>,
}
pub type InterpreterBreakReason = serde_json::Value;
pub type UarchInterpreterBreakReason = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyOfMachineConfigMachineRuntimeConfigStringDoaGddGAMachineRuntimeConfigStringDoaGddGAUnsignedIntegerUnsignedIntegerAccessLogTypeBooleanVyG3AEThAccessLogMachineRuntimeConfigBooleanVyG3AEThAccessLogAccessLogAccessLogMachineRuntimeConfigBooleanVyG3AEThUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerBase64StringUnsignedIntegerUnsignedIntegerUnsignedIntegerBase64StringMemoryRangeConfigCSRCSRUnsignedIntegerCSRUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerStringDoaGddGABooleanVyG3AEThSemanticVersionBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThInterpreterBreakReasonUarchInterpreterBreakReasonAccessLogBooleanVyG3AEThBooleanVyG3AEThProofBase64HashProofUnsignedIntegerBase64StringBooleanVyG3AEThBase64StringBooleanVyG3AEThBooleanVyG3AEThUnsignedIntegerBooleanVyG3AEThUnsignedIntegerUnsignedIntegerUnsignedIntegerUnsignedIntegerBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThUnsignedIntegerUnsignedIntegerUnsignedIntegerBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThUnsignedIntegerBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AEThMachineConfigMachineConfigBooleanVyG3AEThBooleanVyG3AEThBooleanVyG3AETh
{
    MachineConfig(MachineConfig),
    MachineRuntimeConfig(MachineRuntimeConfig),
    StringDoaGddGA(StringDoaGddGA),
    UnsignedInteger(UnsignedInteger),
    AccessLogType(AccessLogType),
    BooleanVyG3AETh(BooleanVyG3AETh),
    AccessLog(AccessLog),
    Base64String(Base64String),
    MemoryRangeConfig(MemoryRangeConfig),
    CSR(CSR),
    SemanticVersion(SemanticVersion),
    InterpreterBreakReason(InterpreterBreakReason),
    UarchInterpreterBreakReason(UarchInterpreterBreakReason),
    Proof(Proof),
    Base64Hash(Base64Hash),
}
#[derive(Clone)]
pub struct RemoteCartesiMachine<T> {
    transport: Box<T>,
}

impl<T: jsonrpsee::core::client::ClientT + Send + Sync> RemoteCartesiMachine<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(transport: T) -> Self {
        RemoteCartesiMachine {
            transport: Box::new(transport),
        }
    }

    pub fn CheckConnection<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        let method = "";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn Fork<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send + 'a>> {
        let method = "fork";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn Shutdown<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "shutdown";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn GetVersion<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<SemanticVersion, Error>> + Send + 'a>> {
        let method = "get_version";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineMachineConfig<'a>(
        &'a self,
        config: MachineConfig,
        runtime: MachineRuntimeConfig,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, jsonrpsee::core::Error>> + Send + 'a>>
    {
        let method = "machine.machine.config";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(config).unwrap();
        params.insert(runtime).unwrap();
        self.transport.request(method, params)
    }

    pub fn MachineMachineDirectory<'a>(
        &'a self,
        directory: StringDoaGddGA,
        runtime: MachineRuntimeConfig,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, jsonrpsee::core::Error>> + Send + 'a>>
    {
        let method = "machine.machine.directory";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(directory).unwrap();
        params.insert(runtime).unwrap();
        self.transport.request(method, params)
    }

    pub fn MachineDestroy<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.destroy";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineStore<'a>(
        &'a mut self,
        directory: StringDoaGddGA,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.store";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(directory).unwrap();
        self.transport.request(method, params)
    }

    pub fn MachineRun<'a>(
        &'a mut self,
        mcycle_end: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<InterpreterBreakReason, Error>> + Send + 'a>> {
        let method = "machine.run";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(mcycle_end).unwrap();
        self.transport.request(method, params)
    }
    
    pub fn MachineRunUarch<'a>(
        &'a mut self,
        uarch_cycle_end: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UarchInterpreterBreakReason, Error>> + Send + 'a>> {
        let method = "machine.run_uarch";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(uarch_cycle_end).unwrap();
        self.transport.request(method, params)
    }

    pub fn MachineStepUarch<'a>(
        &'a mut self,
        log_type: AccessLogType,
        one_based: BooleanVyG3AETh,
    ) -> Pin<Box<dyn Future<Output = Result<AccessLog, Error>> + Send + 'a>> {
        let method = "machine.step_uarch";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(log_type).unwrap();
        params.insert(one_based).unwrap();
        self.transport.request(method, params)
    }

    pub fn MachineVerifyAccessLog<'a>(
        &'a mut self,
        log: AccessLog,
        runtime: MachineRuntimeConfig,
        one_based: BooleanVyG3AETh,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.verify_access_log";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(log);
        params.insert(runtime);
        params.insert(one_based);
        self.transport.request(method, params)
    }

    pub fn MachineVerifyStateTransition<'a>(
        &'a mut self,
        root_hash_before: String,
        log: AccessLog,
        root_hash_after: String,
        runtime: MachineRuntimeConfig,
        one_based: BooleanVyG3AETh,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.verify_state_transition";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(root_hash_before);
        params.insert(log);
        params.insert(root_hash_after);
        params.insert(runtime);
        params.insert(one_based);
        self.transport.request(method, params)
    }

    pub fn MachineGetProof<'a>(
        &'a mut self,
        address: UnsignedInteger,
        log2_size: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<Proof, Error>> + Send + 'a>> {
        let method = "machine.get_proof";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(address);
        params.insert(log2_size);
        self.transport.request(method, params)
    }

    pub fn MachineGetRootHash<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Base64Hash, Error>> + Send + 'a>> {
        let method = "machine.get_root_hash";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineReadWord<'a>(
        &'a mut self,
        address: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.read_word";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(address);
        self.transport.request(method, params)
    }

    pub fn MachineReadMemory<'a>(
        &'a mut self,
        address: UnsignedInteger,
        length: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<Base64String, Error>> + Send + 'a>> {
        let method = "machine.read_memory";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(address);
        params.insert(length);
        self.transport.request(method, params)
    }

    pub fn MachineWriteMemory<'a>(
        &'a mut self,
        address: UnsignedInteger,
        data: Base64String,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.write_memory";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(address);
        params.insert(data);
        self.transport.request(method, params)
    }

    pub fn MachineReadVirtualMemory<'a>(
        &'a mut self,
        address: UnsignedInteger,
        length: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<Base64String, Error>> + Send + 'a>> {
        let method = "machine.read_virtual_memory";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(address);
        params.insert(length);
        self.transport.request(method, params)
    }

    pub fn MachineWriteVirtualMemory<'a>(
        &'a mut self,
        address: UnsignedInteger,
        data: Base64String,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.write_virtual_memory";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(address);
        params.insert(data);
        self.transport.request(method, params)
    }

    pub fn MachineReplaceMemoryRange<'a>(
        &'a mut self,
        range: MemoryRangeConfig,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.replace_memory_range";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(range);
        self.transport.request(method, params)
    }

    pub fn MachineReadCsr<'a>(
        &'a mut self,
        csr: String,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.read_csr";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(csr);
        self.transport.request(method, params)
    }

    pub fn MachineWriteCsr<'a>(
        &'a mut self,
        csr: String,
        value: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.write_csr";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(csr);
        params.insert(value);
        self.transport.request(method, params)
    }

    pub fn MachineGetCsrAddress<'a>(
        &'a mut self,
        csr: String,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.get_csr_address";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(csr);
        self.transport.request(method, params)
    }

    pub fn MachineReadX<'a>(
        &'a mut self,
        index: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.read_x";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        self.transport.request(method, params)
    }

    pub fn MachineReadF<'a>(
        &'a mut self,
        index: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.read_f";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        self.transport.request(method, params)
    }

    pub fn MachineReadUarchX<'a>(
        &'a mut self,
        index: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.read_uarch_x";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        self.transport.request(method, params)
    }

    pub fn MachineWriteX<'a>(
        &'a mut self,
        index: UnsignedInteger,
        value: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.write_x";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        params.insert(value);
        self.transport.request(method, params)
    }

    pub fn MachineWriteF<'a>(
        &'a mut self,
        index: UnsignedInteger,
        value: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.write_f";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        params.insert(value);
        self.transport.request(method, params)
    }

    pub fn MachineWriteUarchX<'a>(
        &'a mut self,
        index: UnsignedInteger,
        value: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.write_uarch_x";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        params.insert(value);
        self.transport.request(method, params)
    }

    pub fn MachineGetXAddress<'a>(
        &'a mut self,
        index: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.get_x_address";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        self.transport.request(method, params)
    }

    pub fn MachineGetFAddress<'a>(
        &'a mut self,
        index: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.get_f_address";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        self.transport.request(method, params)
    }

    pub fn MachineGetUarchXAddress<'a>(
        &'a mut self,
        index: UnsignedInteger,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.get_uarch_x_address";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        params.insert(index);
        self.transport.request(method, params)
    }

    pub fn MachineSetIflagsY<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.set_iflags_Y";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineResetIflagsY<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.reset_iflags_Y";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineReadIflagsY<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.read_iflags_Y";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineSetIflagsX<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.set_iflags_X";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineResetIflagsX<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.reset_iflags_X";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineReadIflagsX<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.read_iflags_X";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineSetIflagsH<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.set_iflags_H";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineReadIflagsH<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.read_iflags_H";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineReadIflagsPRV<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<UnsignedInteger, Error>> + Send + 'a>> {
        let method = "machine.read_iflags_PRV";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineSetUarchHaltFlag<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.set_uarch_halt_flag";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineReadUarchHaltFlag<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.read_uarch_halt_flag";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineResetUarchState<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.reset_uarch_state";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineGetInitialConfig<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<MachineConfig, Error>> + Send + 'a>> {
        let method = "machine.get_initial_config";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineGetDefaultConfig<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<MachineConfig, Error>> + Send + 'a>> {
        let method = "machine.get_default_config";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineVerifyMerkleTree<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.verify_merkle_tree";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineVerifyDirtyPageMaps<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.verify_dirty_page_maps";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }

    pub fn MachineDumpPmas<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<BooleanVyG3AETh, Error>> + Send + 'a>> {
        let method = "machine.dump_pmas";
        let mut params = jsonrpsee::core::params::ArrayParams::new();
        self.transport.request(method, params)
    }
}