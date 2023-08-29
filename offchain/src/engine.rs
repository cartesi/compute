use std::{
    error::Error,
    path::Path,
    sync::{Arc, mpsc},
    thread,
    collections::HashMap,
    time::Duration,
};

use thiserror::Error;
use tokio::{
    sync::Mutex,
    task::JoinSet,
};
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
    player::{Player, PlayerTournamentResult},
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
    players: Vec<Arc<Mutex<Player<A, M>>>>,
}

pub struct Engine <A: Arena, M: Machine> {
    arena: Arc<A>,
    disputes: Arc<Mutex<HashMap<Address, Dispute<A, M>>>>,
    player_config: PlayerConfig,
}

impl<A: Arena + 'static, M: Machine + 'static> Engine<A, M> {
    pub fn new(arena: Arc<A>, player_config: PlayerConfig) -> Self {
        let (player_exec_tx, player_exec_rx) = mpsc::sync_channel::<()>(1);
        Self {
            arena: arena,
            disputes: Arc::new(Mutex::new(HashMap::<Address, Dispute<A,M>>::new())),
            player_config: player_config,
        }
    }

    pub async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        // TODO: spawn as background periodic task with channel for termination.
        
        let mut player_tasks = JoinSet::new();
        let disputes = self.disputes.clone();
        let disputes = disputes.lock().await;
        for (address, dispute) in disputes.iter().filter(|d| !d.1.state.finished) {
            for (player_idx, _) in dispute.players.iter().enumerate() {
                let address = *address;
                player_tasks.spawn_local({
                    let me = self.clone();
                    me.execute_player(address, player_idx)
                });
            }
        }

        Ok(())
    }

    async fn execute_player(self: Arc<Self>, dispute_tournament: Address, player_idx: usize) {
        let player = self.player(dispute_tournament, player_idx).await;
        match player.clone().lock().await.react().await {
            Ok(result) => {
                // TODO: mark dispute as finished
            }
            Err(err) => {
                // TODO: log error
            }
        }
    }

    async fn player(self: Arc<Self>, dispute_tournament: Address, player_idx: usize) -> Arc<Mutex<Player<A,M>>> {
        let disputes = self.disputes.clone();
        let disputes = disputes.lock().await;
        disputes.get(&dispute_tournament).unwrap().players.get(player_idx).unwrap().clone()
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: terminate all cartesi vm processes
        todo!()
    }

    pub async fn start_dispute(
        &mut self,
        inital_hash: Hash,
        machine_snapshot_path: String,
    ) -> Result<Address, Box<dyn Error>> {
        let root_tournament = self.arena.clone().create_root_tournament(inital_hash).await?;  
        let dispute = Dispute {
            state: DisputeState {
                initial_hash: inital_hash,
                machine_snapshot_path: machine_snapshot_path,
                root_tournament: root_tournament,
                finished: false,
            },
            players: Vec::<Arc<Mutex<Player::<A, M>>>>::new(),
        };
        self.disputes.clone().lock().await.insert(root_tournament, dispute);

        Ok(root_tournament)
    }

    pub async fn finish_dispute(
        &mut self,
        root_tournament: Address,
    ) -> Result<DisputeState, Box<dyn Error>> {
        if let Some(dispute_state) = self.disupte_state(root_tournament).await {
            // TODO: terminate all involved cartesi vm processes            
            let disputes = self.disputes.clone();
            disputes.lock().await.remove(&root_tournament);
            Ok(dispute_state.clone())
        } else {
            Err(Box::new(EngineError::DsiputeNotFound(root_tournament.to_string())))
        }
    }

    pub async fn disupte_state(
        &self,
        root_tournament: Address,
    ) -> Option<DisputeState> {
        let disputes = self.disputes.clone();
        let disputes = disputes.lock().await;
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
       let dispute_state = if let Some(dispute_state) = self.disupte_state(root_tournament).await {
            dispute_state
       } else {
            return Err(Box::new(EngineError::DsiputeNotFound(root_tournament.to_string())))
       };

       let machine = self.create_player_machine(dispute_state.machine_snapshot_path).await?;
       
       {
            let disputes = self.disputes.clone();
            let mut disputes = disputes.lock().await;
            if let Some(dispute) = disputes.get_mut(&root_tournament) {
                let player = Player::new(
                    self.arena.clone(),
                    Arc::new(machine),
                    root_tournament,
                );
                dispute.players.push(Arc::new(Mutex::new(player)));
            } else {
                return Err(Box::new(EngineError::DsiputeNotFound(root_tournament.to_string())))
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
