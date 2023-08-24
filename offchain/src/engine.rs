use std::{
    error::Error,
    path::Path,
    sync::{Arc, Mutex, mpsc},
    thread,
    collections::HashMap,
};

use futures::future::join_all;

use thiserror::Error;
use tonic::{Request, Response, Status};

use crate::{
    grpc:: {
        StartDisputeRequest,
        StartDisputeResponse,
        FinishDisputeRequest,
        FinishDisputeResponse,
        GetDisputeInfoRequest,
        GetDisputeInfoResponse,
        JoinDisputeRequest,
        DisputeInfo,
        Compute, JoinDisputeResponse
    },
    config::PlayerConfig,
    arena::{Arena, Hash, Address},
    machine::Machine,
    player::Player,
};

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Dispute with root tournament {0} does not exist")]
    DsiputeNotFound(String),
}

#[derive(Clone)]
pub struct DisputeState {
    initial_hash: Hash,
    machine_snapshot_path: String,
    root_tournament: Address,
    finished: bool,
}

struct Dispute <A: Arena, M: Machine> {
    state: DisputeState,
    players: Vec<Player<A, M>>,
}

pub struct Engine <A: Arena, M: Machine> {
    arena: Arc<A>,
    player_config: PlayerConfig,
    disputes: Arc<Mutex<HashMap<Address, Dispute<A, M>>>>,
}

impl<A: Arena, M: Machine> Engine<A, M> {
    pub fn new(arena: Arc<A>, player_config: PlayerConfig) -> Self {
        Self {
            arena: arena,
            player_config: player_config,
            disputes: Arc::new(Mutex::new(HashMap::<Address, Dispute<A,M>>::new())),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: spawn player execution thread
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: shutdown player execution thread
        Ok(())
    }

    pub async fn start_dispute(
        &mut self,
        inital_hash: Hash,
        machine_snapshot_path: String,
    ) -> Result<Address, Box<dyn Error>> {
        let root_tournament = self.arena.clone().create_root_tournament(inital_hash).await?;  
        {
            let dispute = Dispute {
                state: DisputeState {
                    initial_hash: inital_hash,
                    machine_snapshot_path: machine_snapshot_path,
                    root_tournament: root_tournament,
                    finished: false,
                },
                players: Vec::<Player::<A, M>>::new(),
            };
            self.disputes.clone().lock().unwrap().insert(root_tournament, dispute);
        }

        Ok(root_tournament)
    }

    pub async fn finish_dispute(
        &mut self,
        root_tournament: Address,
    ) -> Result<DisputeState, Box<dyn Error>> {
        let disputes = self.disputes.clone().lock().unwrap();
        if let Some(dispute) = disputes.get(&root_tournament) {
            // TODO: terminate all involved cartesi vm processes
            disputes.remove(&root_tournament);
            Ok(dispute.state.clone())
        } else {
            Err(EngineError::DsiputeNotFound(root_tournament.to_string()))
        }
    }

    pub fn disupte_state(
        &self,
        root_tournament: Address,
    ) -> Option<DisputeState> {
        let disputes = self.disputes.clone().lock().unwrap();
        if let Some(dispute) = disputes.get(&root_tournament) {
            Some(dispute.state.clone())
        } else {
            None
        }
    }

    pub async fn create_player(
        &mut self,
        root_tournament: Address
    ) -> Result<(), Box<dyn Error>> {
       let dispute_state = if let Some(dispute_state) = self.disupte_state(root_tournament) {
            dispute_state
       } else {
            return Err(EngineError::DsiputeNotFound(root_tournament.to_string()))
       };

       let machine = self.create_player_machine(dispute_state.machine_snapshot_path).await?;
       
       {
            let mut disputes = self.disputes.clone().lock().unwrap();
            if let Some(dispute) = disputes.get_mut(&root_tournament) {
                dispute.players.push(Player::new(
                    self.arena.clone(),
                    Arc::new(machine),
                    root_tournament,
                ));
            } else {
                return Err(EngineError::DsiputeNotFound(root_tournament.to_string()))
            }
        }
       
       Ok(())
    }

    async fn create_player_machine(
        &self,
        machine_snapshot_path: String,
    ) -> Result<M, Box<dyn Error>> {
        // TODO:
        // 1. Spawn cartesi vm process.
        // 2. Setup JSON RPC client vm client.
        // 3. Restore vm from snapshot
        todo!()
    }

    async fn execute_players(&mut self) {
        let mut players = Vec::<Player<A, M>>::new();
        {
            let disputes = self.disputes.clone().lock().unwrap();
            disputes.iter().for_each(|(_, dispute)| {
                players.append(&mut dispute.players);
            })
        }

        let player_react_results = players.iter().map(|player| player.react()).collect();
        let player_react_results = join_all(player_react_results);
    }
}

#[tonic::async_trait]
impl<A: Arena + 'static, M: Machine + 'static> Compute for Engine<A, M> {    
    async fn start_dispute(
        &self,
        request: Request<StartDisputeRequest>,
    ) -> Result<Response<StartDisputeResponse>, Status> {
        
        Ok(Response::new(StartDisputeResponse{ dispute_id: String::default() }))
    }

    async fn finish_dispute(
        &self,
        request: Request<FinishDisputeRequest>,
    ) -> Result<Response<FinishDisputeResponse>, Status> {
        Ok(Response::new(FinishDisputeResponse{ 
            dispute_info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }

    async fn get_dispute_info(
        &self,
        request: Request<GetDisputeInfoRequest>,
    ) -> Result<Response<GetDisputeInfoResponse>, Status> {
        Ok(Response::new(GetDisputeInfoResponse{
            dispute_info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }

    async fn join_dispute(
        &self,
        request: Request<JoinDisputeRequest>,
    ) -> Result<Response<JoinDisputeResponse>, Status> {
        Ok(Response::new(JoinDisputeResponse {
            dispute_info: Some(DisputeInfo {
                closed: false,
            }),
        }))
    }
}
