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

extern crate protobuf;

use super::dispatcher::{Archive, Reaction};
use super::dispatcher::DApp;
use super::dispatcher::{U256Array2, Bytes32Array3, AddressArray};
use super::error::*;
use super::ethabi::Token;
use super::ethereum_types::{H256, U256, Address};
use super::transaction;
use super::transaction::TransactionRequest;
use super::compute::vg::{VG, VGCtx, VGCtxParsed};
use super::Role;

pub struct Descartes();

#[derive(Serialize, Deserialize)]
pub enum TupleType {
    #[serde(rename = "(bytes32,uint64,uint64,bytes32,address,uint8)[]")]
    DriveArrayType,
}

#[derive(Serialize, Deserialize)]
pub struct DriveParsed(
    H256,   // driveHash
    U256,   // position
    U256,   // log2Size
    H256,   // bytes32Value
    Address,// provider
    U256,   // driveType
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
    drive_hash: H256,
    position: U256,
    log2_size: U256,
    value: H256,
    provider: Address,
    drive_type: U256,
}

impl From<&DriveParsed> for Drive {
    fn from(parsed: &DriveParsed) -> Drive {
        Drive {
            drive_hash: parsed.0,
            position: parsed.1,
            log2_size: parsed.2,
            value: parsed.3,
            provider: parsed.4,
            drive_type: parsed.5,
        }
    }
}
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// these two structs and the From trait below shuld be
// obtained from a simple derive
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
#[derive(Serialize, Deserialize)]
pub struct DescartesCtxParsed(
    U256Array2, // finalTime, deadline
    AddressArray, // challenger, claimer
    Bytes32Array3, // initialHash, claimedFinalHash, currentState
    DriveArray,
);

#[derive(Serialize, Debug)]
pub struct DescartesCtx {
    pub initial_hash: H256,
    pub claimed_final_hash: H256,
    pub claimer: Address,
    pub challenger: Address,
    pub deadline: U256,
    pub final_time: U256,
    pub current_state: String,
    pub drives: Vec<Drive>,
}

impl From<DescartesCtxParsed> for DescartesCtx {
    fn from(parsed: DescartesCtxParsed) -> DescartesCtx {
        DescartesCtx {
            final_time: parsed.0.value[0],
            deadline: parsed.0.value[1],
            challenger: parsed.1.value[0],
            claimer: parsed.1.value[1],
            initial_hash: parsed.2.value[0],
            claimed_final_hash: parsed.2.value[1],
            current_state: parsed.2.value[2].to_string(),
            drives: parsed.3.value.iter().map(|d| d.into()).collect(),
        }
    }
}

impl DApp<()> for Descartes {
    /// React to the descartes contract, submitting drives, 
    /// submitting solutions, confirming or challenging them
    /// when appropriate
    fn react(
        instance: &state::Instance,
        archive: &Archive,
        post_action: &Option<String>,
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
            "ClaimerMissedDeadline" | "ChallengerWon" | "ClaimerWon" | "ConsensusResult" => {
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

        match role {
            Role::Claimer => match ctx.current_state.as_ref() {
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
                            // claim victory in compute contract
                            info!("Claiming victory for Compute (index: {})", instance.index);
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
                            let machine_id = "test_machine".to_string();
                            return VG::react(vg_instance, archive, &None, &machine_id);
                        }
                    }
                },
                _ => {
                    return Ok(Reaction::Idle);
                }
            },
            _ => {
                return Ok(Reaction::Idle);
            }
        };
    }

    fn get_pretty_instance(
        instance: &state::Instance,
        archive: &Archive,
        _: &(),
    ) -> Result<state::Instance> {
        // get context (state) of the arbitration test instance
        let parsed: DescartesCtxParsed =
            serde_json::from_str(&instance.json_data).chain_err(|| {
                format!(
                    "Could not parse arbitration test instance json_data: {}",
                    &instance.json_data
                )
            })?;
        let ctx: DescartesCtx = parsed.into();
        let json_data = serde_json::to_string(&ctx).unwrap();

        // get context (state) of the sub instances

        let mut pretty_sub_instances: Vec<Box<state::Instance>> = vec![];

        let machine_id = "test_machine".to_string();
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
