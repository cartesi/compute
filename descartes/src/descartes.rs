// Copyright (C) 2020 Cartesi Pte. Ltd.

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Note: This component currently has dependencies that are licensed under the GNU
// GPL, version 3, and so you should treat this component as a whole as being under
// the GPL version 3. But all Cartesi-written code in this component is licensed
// under the Apache License, version 2, or a compatible permissive license, and can
// be used independently under the Apache v2 license. After this component is
// rewritten, the entire component will be released under the Apache v2 license.

use super::configuration::Concern;
use super::dispatcher::{Archive, Reaction};
use super::dispatcher::DApp;
use super::dispatcher::{U256Array, Bytes32Array, AddressArray};
use super::error::*;
use super::ethabi::Token;
use super::ethereum_types::{H256, U256, Address};
use super::transaction;
use super::transaction::TransactionRequest;
use super::compute::vg::{VG, VGCtx, VGCtxParsed};

pub use compute::{
    get_run_result, cartesi_machine, build_session_write_key,
    build_session_proof_key, build_session_read_key, build_session_run_key,
    SessionRunRequest, SessionReadMemoryRequest, SessionReadMemoryResponse,
    SessionWriteMemoryRequest, NewSessionRequest, NewSessionResponse,
    SessionGetProofRequest, SessionGetProofResponse, SessionRunResult,
    EMULATOR_SERVICE_NAME, EMULATOR_METHOD_WRITE, EMULATOR_METHOD_READ,
    EMULATOR_METHOD_PROOF, EMULATOR_METHOD_NEW
};

pub use logger_service::{
    DownloadFileRequest, DownloadFileResponse,
    SubmitFileRequest, SubmitFileResponse,
    LOGGER_METHOD_DOWNLOAD, LOGGER_METHOD_SUBMIT,
};
use super::{Role, build_machine_id, get_logger_response};

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Descartes();


#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    pub action: String,
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    // value is the hash of a drive if is a LoggerDrive
    // or is the exact value of a DirectDrive
    pub value: H256,
}

#[derive(Serialize, Deserialize)]
pub enum TupleType {
    #[serde(rename = "(uint64,uint64,bytes32,address,bool,bool)[]")]
    DriveArrayType,
}

#[derive(Serialize, Deserialize)]
pub struct DriveParsed(
    U256,   // position
    U256,   // log2Size
    H256,   // bytes32Value
    Address,// provider
    bool,   // needsProvider
    bool,   // needsLogger
);

#[derive(Serialize, Deserialize)]
pub struct DriveArray {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: TupleType,
    pub value: Vec<DriveParsed>,
}

#[derive(Serialize, Debug)]
pub struct Drive {
    position: U256,
    log2_size: U256,
    value: H256,
    provider: Address,
    needs_provider: bool,
    needs_logger: bool,
}

impl From<&DriveParsed> for Drive {
    fn from(parsed: &DriveParsed) -> Drive {
        Drive {
            position: parsed.0,
            log2_size: parsed.1,
            value: parsed.2,
            provider: parsed.3,
            needs_provider: parsed.4,
            needs_logger: parsed.5,
        }
    }
}

impl From<&Drive> for Token {
    fn from(drive: &Drive) -> Token {
        Token::Tuple(vec![
            Token::Uint(drive.position),
            Token::Uint(drive.log2_size),
            Token::FixedBytes(drive.value.to_fixed_bytes().to_vec()),
            Token::Address(drive.provider),
            Token::Bool(drive.needs_provider),
            Token::Bool(drive.needs_logger),
        ])
    }
}
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// these two structs and the From trait below shuld be
// obtained from a simple derive
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
#[derive(Serialize, Deserialize)]
pub struct DescartesCtxParsed(
    U256Array, // finalTime, deadline
    AddressArray, // challenger, claimer
    Bytes32Array, // initialHash, claimedFinalHash, claimedOutput, currentState
    DriveArray,
);

#[derive(Serialize, Debug)]
pub struct DescartesCtx {
    pub initial_hash: H256,
    pub claimed_final_hash: H256,
    pub claimed_output: H256,
    pub claimer: Address,
    pub challenger: Address,
    pub deadline: U256,
    pub output_position: U256,
    pub final_time: U256,
    pub current_state: String,
    pub input_drives: Vec<Drive>,
}

impl From<DescartesCtxParsed> for DescartesCtx {
    fn from(parsed: DescartesCtxParsed) -> DescartesCtx {
        DescartesCtx {
            final_time: parsed.0.value[0],
            deadline: parsed.0.value[1],
            output_position: parsed.0.value[2],
            challenger: parsed.1.value[0],
            claimer: parsed.1.value[1],
            initial_hash: parsed.2.value[0],
            claimed_final_hash: parsed.2.value[1],
            claimed_output: parsed.2.value[2],
            current_state: String::from_utf8(
                    parsed.2.value[3]
                        .to_fixed_bytes()
                        .to_vec()
                        .iter()
                        .take_while(|&n| *n != 0)
                        .map(|&n| n)
                        .collect()
                    )
                .unwrap()
                .to_string(),
            input_drives: parsed.3.value.iter().map(|d| d.into()).collect(),
        }
    }
}

impl DApp<()> for Descartes {
    /// React to the descartes contract, submitting drives, 
    /// submitting result, confirming or challenging result
    /// when appropriate
    fn react(
        instance: &state::Instance,
        archive: &Archive,
        post_payload: &Option<String>,
        _: &(),
    ) -> Result<Reaction> {
        // get context (state) of the Descartes instance
        let parsed: DescartesCtxParsed =
            serde_json::from_str(&instance.json_data).chain_err(|| {
                format!(
                    "Could not parse descartes instance json_data: {}",
                    &instance.json_data
                )
            })?;
        let ctx: DescartesCtx = parsed.into();
        trace!("Context for descartes (index {}) {:?}", instance.index, ctx);

        // these states should not occur as they indicate an innactive instance,
        // but it is possible that the blockchain state changed between queries
        match ctx.current_state.as_ref() {
            "ProviderMissedDeadline" |
            "ClaimerMissedDeadline" |
            "ChallengerWon" |
            "ClaimerWon" |
            "ConsensusResult" => {
                return Ok(Reaction::Idle);
            }
            _ => {}
        };

        // if we reach this code, the instance is active, get user's role
        let role = match instance.concern.user_address {
            cl if (cl == ctx.claimer) => Role::Claimer,
            ch if (ch == ctx.challenger) => Role::Challenger,
            _ => {
                return Err(Error::from(ErrorKind::InvalidContractState(String::from(
                    "User is neither claimer nor challenger",
                ))));
            }
        };
        trace!("Role played (index {}) is: {:?}", instance.index, role);

        match ctx.current_state.as_ref() {
            "WaitingProviders" => {
                if let Some(s) = post_payload {
                    // got request from user to claim a pending drive
                    let payload: Payload = serde_json::from_str(&s)
                    .chain_err(|| format!("Could not parse post_payload: {}", &s))?;

                    let mut function = String::from("claimLoggerDrive");
                    if !ctx.input_drives[0].needs_logger {
                        function = String::from("claimDirectDrive");
                    } else {
                        // the file name should be default to the root hash
                        let request = SubmitFileRequest {
                            path: format!("{:x}", payload.params.value.clone()),
                            page_log2_size: 3,
                            tree_log2_size: ctx.input_drives[0].log2_size.as_u64(),
                        };

                        let processed_response: SubmitFileResponse = get_logger_response(
                            archive,
                            "Descartes".into(),
                            format!("{:x}", payload.params.value.clone()),
                            LOGGER_METHOD_SUBMIT.to_string(),
                            request.into(),
                        )?
                        .into();
                        if processed_response.root != payload.params.value {
                            error!("Submitted log hash({:x}) doesn't match value from post_payload{:x}",
                                processed_response.root,
                                payload.params.value);
                            return Ok(Reaction::Idle);
                        }
                    }
                    let request = TransactionRequest {
                        contract_name: None, // Name not needed, is concern
                        concern: instance.concern.clone(),
                        value: U256::from(0),
                        function: function,
                        data: vec![
                            Token::Uint(instance.index),
                            Token::FixedBytes(
                                payload.params.value
                                    .to_fixed_bytes()
                                    .to_vec())
                            ],
                        gas: None,
                        strategy: transaction::Strategy::Simplest,
                    };
                    return Ok(Reaction::Transaction(request));
                };
                if instance.concern.user_address != ctx.input_drives[0].provider {
                    // wait others to provide drives
                    // or abort if the deadline is over
                    return abort_by_deadline_or_idle(
                        &instance.concern,
                        instance.index,
                        ctx.deadline.as_u64(),
                    );
                }
            },
            _ => {}
        };

        match role {
            Role::Claimer => match ctx.current_state.as_ref() {
                "WaitingClaim" => {
                    // calculate machine output
                    return react_by_machine_output(
                        archive,
                        &instance.concern,
                        instance.index,
                        &role,
                        ctx.input_drives,
                        ctx.initial_hash,
                        ctx.claimed_final_hash,
                        ctx.final_time,
                        ctx.output_position,
                    );
                },
                "WaitingChallenge" => {
                    // we inspect the verification contract
                    let vg_instance = instance.sub_instances.get(0).ok_or(Error::from(
                        ErrorKind::InvalidContractState(format!(
                            "There is no vg instance {}",
                            ctx.current_state
                        )),
                    ))?;
                    let vg_parsed: VGCtxParsed = serde_json::from_str(&vg_instance.json_data)
                        .chain_err(|| {
                            format!(
                                "Could not parse vg instance json_data: {}",
                                &vg_instance.json_data
                            )
                        })?;
                    let vg_ctx: VGCtx = vg_parsed.into();

                    match vg_ctx.current_state.as_ref() {
                        "FinishedClaimerWon" => {
                            // claim victory in descartes contract
                            info!("Claiming victory for Descartes (index: {})", instance.index);
                            let request = TransactionRequest {
                                contract_name: None, // Name not needed, is concern
                                concern: instance.concern.clone(),
                                value: U256::from(0),
                                function: "winByVG".into(),
                                data: vec![Token::Uint(instance.index)],
                                gas: None,
                                strategy: transaction::Strategy::Simplest,
                            };
                            return Ok(Reaction::Transaction(request));
                        }
                        "FinishedChallengerWon" => {
                            error!("we lost a verification game {:?}", vg_ctx);
                            return Ok(Reaction::Idle);
                        }
                        _ => {
                            // verification game is still active,
                            // pass control to the appropriate dapp
                            let machine_id = build_machine_id(instance.index, &instance.concern.user_address);
                            return VG::react(vg_instance, archive, &None, &machine_id);
                        }
                    }
                },
                _ => {
                    return Ok(Reaction::Idle);
                }
            },
            Role::Challenger => match ctx.current_state.as_ref() {
                "WaitingClaim" => {
                    // wait for the claimer to claim output
                    // or abort if the deadline is over
                    return abort_by_deadline_or_idle(
                        &instance.concern,
                        instance.index,
                        ctx.deadline.as_u64(),
                    );
                }
                "WaitingConfirmation" => {
                    // determine the reaction based on the calculated machine output
                    return react_by_machine_output(
                        archive,
                        &instance.concern,
                        instance.index,
                        &role,
                        ctx.input_drives,
                        ctx.initial_hash,
                        ctx.claimed_final_hash,
                        ctx.final_time,
                        ctx.output_position,
                    );
                },
                "WaitingChallenge" => {
                    // we inspect the verification contract
                    let vg_instance = instance.sub_instances.get(0).ok_or(Error::from(
                        ErrorKind::InvalidContractState(format!(
                            "There is no vg instance {}",
                            ctx.current_state
                        )),
                    ))?;
                    let vg_parsed: VGCtxParsed = serde_json::from_str(&vg_instance.json_data)
                        .chain_err(|| {
                            format!(
                                "Could not parse vg instance json_data: {}",
                                &vg_instance.json_data
                            )
                        })?;
                    let vg_ctx: VGCtx = vg_parsed.into();

                    match vg_ctx.current_state.as_ref() {
                        "FinishedChallengerWon" => {
                            // claim victory in descartes contract
                            info!("Claiming victory for Descartes (index: {})", instance.index);
                            let request = TransactionRequest {
                                contract_name: None, // Name not needed, is concern
                                concern: instance.concern.clone(),
                                value: U256::from(0),
                                function: "winByVG".into(),
                                data: vec![Token::Uint(instance.index)],
                                gas: None,
                                strategy: transaction::Strategy::Simplest,
                            };
                            return Ok(Reaction::Transaction(request));
                        }
                        "FinishedClaimerWon" => {
                            error!("we lost a verification game {:?}", vg_ctx);
                            return Ok(Reaction::Idle);
                        }
                        _ => {
                            // verification game is still active,
                            // pass control to the appropriate dapp
                            let machine_id = build_machine_id(instance.index, &instance.concern.user_address);
                            return VG::react(vg_instance, archive, &None, &machine_id);
                        }
                    }
                },
                _ => {
                    return Ok(Reaction::Idle);
                }
            },
        };
    }

    fn get_pretty_instance(
        instance: &state::Instance,
        archive: &Archive,
        _: &(),
    ) -> Result<state::Instance> {
        // get context (state) of the descartes instance
        let parsed: DescartesCtxParsed =
            serde_json::from_str(&instance.json_data).chain_err(|| {
                format!(
                    "Could not parse descartes instance json_data: {}",
                    &instance.json_data
                )
            })?;
        let ctx: DescartesCtx = parsed.into();
        let json_data = serde_json::to_string(&ctx).unwrap();

        // get context (state) of the sub instances

        let mut pretty_sub_instances: Vec<Box<state::Instance>> = vec![];

        let machine_id = build_machine_id(instance.index, &instance.concern.user_address);
        for sub in &instance.sub_instances {
            pretty_sub_instances.push(Box::new(
                VG::get_pretty_instance(sub, archive, &machine_id).unwrap(),
            ))
        }

        let pretty_instance = state::Instance {
            name: "Descartes".to_string(),
            concern: instance.concern.clone(),
            index: instance.index,
            service_status: archive.get_service("Descartes".into()),
            json_data: json_data,
            sub_instances: pretty_sub_instances,
        };

        return Ok(pretty_instance);
    }
}

fn abort_by_deadline_or_idle(
    concern: &Concern,
    index: U256,
    deadline: u64,
) -> Result<Reaction> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .chain_err(|| "System time before UNIX_EPOCH")?
        .as_secs();

    // if other party missed the deadline
    if current_time > deadline {
        info!("Aborting instance by time (index: {})", index);
        let request = TransactionRequest {
            contract_name: None, // Name not needed, is concern
            concern: concern.clone(),
            value: U256::from(0),
            function: "abortByDeadline".into(),
            data: vec![Token::Uint(index)],
            gas: None,
            strategy: transaction::Strategy::Simplest,
        };
        return Ok(Reaction::Transaction(request));
    } else {
        // if not, then wait
        return Ok(Reaction::Idle);
    }
}

fn react_by_machine_output(
    archive: &Archive,
    concern: &Concern,
    index: U256,
    role: &Role,
    input_drives: Vec<Drive>,
    template_hash: H256,
    claimed_final_hash: H256,
    final_time: U256,
    output_position: U256,
) -> Result<Reaction> {
    // create machine and fill in all the drives
    let id = build_machine_id(index, &concern.user_address);

    let mut machine = cartesi_machine::MachineRequest::new();
    machine.set_directory(format!("/opt/cartesi/srv/descartes/cartesi-machine/{:x}", template_hash));
    
    let request = NewSessionRequest {
        session_id: id.clone(),
        machine: machine,
    };

    // send newSession request to the emulator service
    let _processed_response: NewSessionResponse = archive
        .get_response(
            EMULATOR_SERVICE_NAME.to_string(),
            id.clone(),
            EMULATOR_METHOD_NEW.to_string(),
            request.into(),
        )?
        .into();

    let mut drives_siblings = vec![];
    let mut output_siblings = vec![];
    let mut calculated_output = H256::zero();
    
    let time = 0;
    for drive in &input_drives {
        let address = drive.position.as_u64();
        let log2_size = drive.log2_size.as_u64();
        if !drive.needs_logger {
            // write direct values to drive
            let data = drive.value.to_fixed_bytes();
            let archive_key = build_session_write_key(id.clone(), time, address, data.to_vec());

            let mut position = cartesi_machine::WriteMemoryRequest::new();
            position.set_address(address);
            position.set_data(data.to_vec());

            let request = SessionWriteMemoryRequest {
                session_id: id.clone(),
                time: time,
                position: position,
            };

            let _processed_response = archive
                .get_response(
                    EMULATOR_SERVICE_NAME.to_string(),
                    archive_key.clone(),
                    EMULATOR_METHOD_WRITE.to_string(),
                    request.into(),
                )?;
        } else {
            let downloaded_drive = format!("/opt/cartesi/srv/descartes/flashdrive/{:x}", drive.value.clone());
            let request = DownloadFileRequest {
                root: drive.value.clone(),
                path: downloaded_drive.clone(),
                page_log2_size: 3,
                tree_log2_size: drive.log2_size.as_u64(),
            };

            // mounted volume in dispatcher and logger is different path
            // logger is at /opt/cartesi/srv/logger-server/
            // dispatcher is at /opt/cartesi/srv/descartes/
            let processed_response: DownloadFileResponse = get_logger_response(
                archive,
                "Descartes".into(),
                format!("{:x}", drive.value.clone()),
                LOGGER_METHOD_DOWNLOAD.to_string(),
                request.into(),
            )?
            .into();
            trace!("Downloaded! File stored at: {}...", processed_response.path);

            // TODO: rewrite with flash replacement call later
            
            let data = std::fs::read(downloaded_drive)?;
            let archive_key = build_session_write_key(id.clone(), time, address, data.clone());

            let mut position = cartesi_machine::WriteMemoryRequest::new();
            position.set_address(address);
            position.set_data(data);

            let request = SessionWriteMemoryRequest {
                session_id: id.clone(),
                time: time,
                position: position,
            };

            let _processed_response = archive
                .get_response(
                    EMULATOR_SERVICE_NAME.to_string(),
                    archive_key.clone(),
                    EMULATOR_METHOD_WRITE.to_string(),
                    request.into(),
                )?;
        }
        if let Role::Claimer = role {
            // get input drive siblings
            let archive_key = build_session_proof_key(id.clone(), time, address, log2_size);
            let mut target = cartesi_machine::GetProofRequest::new();
            target.set_address(address);
            target.set_log2_size(log2_size);
    
            let request = SessionGetProofRequest {
                session_id: id.clone(),
                time: time,
                target: target,
            };
    
            let processed_response: SessionGetProofResponse = archive
                .get_response(
                    EMULATOR_SERVICE_NAME.to_string(),
                    archive_key.clone(),
                    EMULATOR_METHOD_PROOF.to_string(),
                    request.into(),
                )?
                .into();
    
            trace!("Get proof result: {:?}...", processed_response.proof);
    
            // get actual siblings
            let mut drive_siblings: Vec<_> = processed_response
                .proof
                .sibling_hashes
                .into_iter()
                .map(|hash| Token::FixedBytes(hash.0.to_vec()))
                .collect();
            trace!("Size of siblings: {}", drive_siblings.len());
            // !!!!! This should not be necessary, !!!!!!!
            // !!!!! the emulator should do it     !!!!!!!
            drive_siblings.reverse();

            drives_siblings.push(Token::Array(drive_siblings));
        }
    }

    let time = final_time.as_u64();
    let sample_points: Vec<u64> = vec![0, time];

    let request = SessionRunRequest {
        session_id: id.clone(),
        times: sample_points.clone(),
    };
    let archive_key = build_session_run_key(id.clone(), sample_points.clone());

    let processed_result: SessionRunResult = get_run_result(
        archive,
        "Descartes".to_string(),
        archive_key,
        request.into(),
    )?;

    let calculated_final_hash = processed_result.hashes[1];

    if let Role::Claimer = role {
        // get output value
        let length = 32;
        let address = output_position.as_u64();
    
        let archive_key = build_session_read_key(id.clone(), time, address, length);
        let mut position = cartesi_machine::ReadMemoryRequest::new();
        position.set_address(address);
        position.set_length(length);
    
        let request = SessionReadMemoryRequest {
            session_id: id.clone(),
            time: time,
            position: position,
        };
    
        let processed_response: SessionReadMemoryResponse = archive
            .get_response(
                EMULATOR_SERVICE_NAME.to_string(),
                archive_key.clone(),
                EMULATOR_METHOD_READ.to_string(),
                request.into(),
            )?
            .into();
    
        trace!(
            "Read memory result: {:?}...",
            processed_response.read_content.data
        );
    
        calculated_output = H256::from_slice(&processed_response.read_content.data);

        // get output drive siblings
        let output_logsize_2 = 5;

        let archive_key = build_session_proof_key(id.clone(), time, address, output_logsize_2);
        let mut target = cartesi_machine::GetProofRequest::new();
        target.set_address(address);
        target.set_log2_size(output_logsize_2);

        let request = SessionGetProofRequest {
            session_id: id.clone(),
            time: time,
            target: target,
        };

        let processed_response: SessionGetProofResponse = archive
            .get_response(
                EMULATOR_SERVICE_NAME.to_string(),
                archive_key.clone(),
                EMULATOR_METHOD_PROOF.to_string(),
                request.into(),
            )?
            .into();

        trace!("Get proof result: {:?}...", processed_response.proof);

        // get actual siblings
        output_siblings = processed_response
            .proof
            .sibling_hashes
            .into_iter()
            .map(|hash| Token::FixedBytes(hash.0.to_vec()))
            .collect();
        trace!("Size of siblings: {}", output_siblings.len());
        // !!!!! This should not be necessary, !!!!!!!
        // !!!!! the emulator should do it     !!!!!!!
        output_siblings.reverse();
    }

    match role {
        Role::Claimer => {
            info!("Claiming output (index: {})", index);
            let request = TransactionRequest {
                contract_name: None, // Name not needed, is concern
                concern: concern.clone(),
                value: U256::from(0),
                function: "submitClaim".into(),
                data: vec![
                    Token::Uint(index),
                    Token::FixedBytes(calculated_final_hash.to_fixed_bytes().to_vec()),
                    Token::Array(input_drives.iter().map(|d:&Drive| -> Token {d.into()}).collect()),
                    Token::Array(drives_siblings),
                    Token::FixedBytes(calculated_output.to_fixed_bytes().to_vec()),
                    Token::Array(output_siblings),
                ],
                // TODO: change back to None after done testing
                gas: Some(U256::from(628318)),
                strategy: transaction::Strategy::Simplest,
            };
            return Ok(Reaction::Transaction(request));
        },
        Role::Challenger => {
            let mut function = String::from("challenge");
            if calculated_final_hash == claimed_final_hash {
                function = String::from("confirm");
            }

            let request = TransactionRequest {
                contract_name: None, // Name not needed, is concern
                concern: concern.clone(),
                value: U256::from(0),
                function: function,
                data: vec![Token::Uint(index)],
                gas: None,
                strategy: transaction::Strategy::Simplest,
            };
            return Ok(Reaction::Transaction(request));
        },
    }
}