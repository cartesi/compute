mod compute;

pub use compute::{
    StartDisputeRequest,
    StartDisputeResponse,
    FinishDisputeRequest,
    FinishDisputeResponse,
    GetDisputeInfoRequest,
    GetDisputeInfoResponse,
    JoinDisputeRequest,
    JoinDisputeResponse,
    DisputeInfo,
    MachineSetup,
};

pub use compute::compute_server::{ 
    Compute, 
    ComputeServer,
};