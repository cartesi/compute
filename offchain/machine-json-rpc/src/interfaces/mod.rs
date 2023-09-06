mod index;
pub use index::*;


#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct NewSessionRequest {
    pub machine_config: index::MachineConfig,
    pub machine_runtime_config: index::MachineRuntimeConfig,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct MachineRequest {
    pub runtime: Option<index::MachineRuntimeConfig>,
    pub machine_oneof: Option<MachineOneof>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub enum MachineOneof {
    Config(index::MachineConfig),
    Directory(String),
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionRunRequest {
    pub session_id: String,
    pub final_cycles: Vec<u64>,
    pub final_ucycles: Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SessionRunProgress {
    pub progress: u64,
    pub application_progress: u64,
    pub updated_at: u64,
    pub cycle: u64,
    pub ucycle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SessionRunResult {
    pub summaries: Vec<RunResponse>,
    pub hashes: Vec<Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RunResponse {
    pub mcycle: u64,
    pub tohost: u64,
    pub iflags_h: bool,
    pub iflags_y: bool,
    pub iflags_x: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default)]
pub struct SessionRunResponse {
    pub run_oneof: Option<session_run_response::RunOneof>,
}
/// Nested message and enum types in `SessionRunResponse`.
pub mod session_run_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq)]
    pub enum RunOneof {
        Progress(super::SessionRunProgress),
        Result(super::SessionRunResult),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionStepRequest {
    pub session_id: String,
    pub initial_cycle: u64,
    pub initial_ucycle: u64,
    pub step_params_oneof: Option<session_step_request::StepParamsOneof>,
}
/// Nested message and enum types in `SessionStepRequest`.
pub mod session_step_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Debug)]
    pub enum StepParamsOneof {
        StepParams(crate::interfaces::StepUarchRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct StepUarchRequest {
    pub log_type: Option<index::AccessLogType>,
    pub one_based: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionStepResponse {
    pub log: Option<index::AccessLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionStoreRequest {
    pub session_id: String,
    pub store: Option<crate::interfaces::StoreRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct StoreRequest {
    pub directory: String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionReadMemoryRequest {
    pub session_id: String,
    pub cycle: u64,
    pub position: Option<ReadMemoryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct ReadMemoryRequest {
    pub address: u64,
    pub length: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SessionReadMemoryResponse {
    pub read_content: Option<ReadMemoryResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ReadMemoryResponse {
    pub data: Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionReplaceMemoryRangeRequest {
    pub session_id: String,
    pub cycle: u64,
    pub ucycle: u64,
    pub range: Option<index::MemoryRangeConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionWriteMemoryRequest {
    pub session_id: String,
    pub cycle: u64,
    pub ucycle: u64,
    pub position: Option<WriteMemoryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct WriteMemoryRequest {
    pub address: u64,
    pub data: Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SessionGetProofRequest {
    pub session_id: String,
    pub cycle: u64,
    pub ucycle: u64,
    pub target: Option<GetProofRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct GetProofRequest {
    pub address: u64,
    pub log2_size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default)]
pub struct EndSessionRequest {
    pub session_id: String,
    pub silent: bool,
}