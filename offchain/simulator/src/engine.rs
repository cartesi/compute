use std::{
    error::Error,
    path::Path,
    sync::Arc,
    collections::HashMap,
    time::Duration,
};

use thiserror::Error;
use tokio::{
    sync::Mutex,
    task,
    time,
    select,
};
use tokio_util::sync::CancellationToken;
use tonic::transport::channel::Channel;

use cartesi_compute_core::{
    merkle::Hash,
    machine::{
        MachineRpc,
        MachineFactory,
        MachineCommitmentBuilder,
        CachingMachineCommitmentBuilder,
        FakeMachineCommitmentBuilder,
    },
    arena::{Address, Arena},
    player::{Player, PlayerTournamentResult},
};

use cartesi_compute_coordinator::grpc::StartDisputeRequest;

pub type CoordinatorClient = cartesi_compute_coordinator::grpc::CoordinatorClient<Channel>;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Dispute with root tournament {0} does not exist")]
    DsiputeNotFound(String),
}

#[derive(Clone)]
pub struct DisputeState {
    pub initial_hash: Hash,
    pub machine_snapshot_path: String,
    pub root_tournament: Address,
    pub finished: bool,
}

struct Dispute <A: Arena,> {
    state: DisputeState,
    players: Vec<Arc<Mutex<Player<A>>>>,
}

pub struct EngineConfig {
    pub player_react_period: Duration, 
}

pub struct Engine <A: Arena> {
    coordinator: Arc<Mutex<CoordinatorClient>>,
    machine_factory: Arc<Mutex<MachineFactory>>,
    config: EngineConfig,
    disputes: Arc<Mutex<HashMap<Address, Dispute<A>>>>,
    shutdown_token: CancellationToken,
}

impl<A: Arena + 'static> Engine<A> {
    pub fn new(
        coordinator: Arc<Mutex<CoordinatorClient>>,
        machine_facotry: Arc<Mutex<MachineFactory>>,
        config: EngineConfig) -> Self {
        Self {
            coordinator: coordinator,
            machine_factory: machine_facotry,
            config,
            disputes: Arc::new(Mutex::new(HashMap::<Address, Dispute<A>>::new())),
            shutdown_token: CancellationToken::new(),
        }
    }

    pub async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        let palyer_executor = task::spawn_local(async move{
            let shutdown = self.shutdown_token.clone();
            let mut exec_timer = time::interval(self.config.player_react_period);
            loop {
                select! {
                    _ = shutdown.cancelled() => return Ok(()),
                    _ = exec_timer.tick() => self.clone().execute_players().await,
                }
            }
        });
        palyer_executor.await?
    }

    async fn execute_players(self: Arc<Self>) {
        let mut player_tasks = task::JoinSet::new();
        
        {
            let disputes = self.disputes.clone();
            let disputes = disputes.lock().await;
            for (address, dispute) in disputes.iter().filter(|d| !d.1.state.finished) {
                for (player_idx, _) in dispute.players.iter().enumerate() {
                    let address = *address;
                    player_tasks.spawn_local({
                        self.clone().execute_player(address, player_idx)
                    });
                }
            }
        }
        
        while let Some(_) = player_tasks.join_next().await {}
    }

    async fn execute_player(self: Arc<Self>, dispute_tournament: Address, player_idx: usize) {
        let result: Result<(Option<PlayerTournamentResult>), Box<dyn std::error::Error>>;
        {
            let player = self.clone().player(dispute_tournament, player_idx).await;
            result = player.clone().lock().await.react().await; 
        };
        
        match  result {
            Ok(result) => {
                if let Err(err) = self.clone().finish_dispute(dispute_tournament).await {
                    // TODO log error
                }
            }
            Err(err) => {
                // TODO: log error
            }
        }
    }

    async fn player(self: Arc<Self>, dispute_tournament: Address, player_idx: usize) -> Arc<Mutex<Player<A>>> {
        let disputes = self.disputes.clone();
        let disputes = disputes.lock().await;
        disputes.get(&dispute_tournament).unwrap().players.get(player_idx).unwrap().clone()
    }

    pub async fn shutdown(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        self.shutdown_token.cancel();
        // TODO: terminate all cartesi vm processes
        Ok(())
    }

    pub async fn start_dispute(
        self: Arc<Self>,
        initial_hash_data: [u8; 32],
        machine_snapshot_path: &String,
    ) -> Result<Address, Box<dyn Error>> {
        let coordinator = self.coordinator.clone();
        let mut coordinator = coordinator.lock().await;
        let response = coordinator.start_dispute(StartDisputeRequest{
            initial_hash: initial_hash_data.into(),
            snapshot_path: machine_snapshot_path.clone(),
        }).await?;

        let root_tournament = response.into_inner().dispute_id.parse::<Address>()?;
        
        let disputes = self.disputes.clone();
        let mut disputes = disputes.lock().await;
        disputes.insert(root_tournament, Dispute {
            state: DisputeState {
                initial_hash: Hash::new(initial_hash_data),
                machine_snapshot_path: machine_snapshot_path.clone(),
                root_tournament: root_tournament,
                finished: false,
            },
            players: Vec::<Arc<Mutex<Player::<A>>>>::new(),
        });

        Ok(root_tournament)
    }

    pub async fn finish_dispute(
        self: Arc<Self>,
        root_tournament: Address,
    ) -> Result<DisputeState, Box<dyn Error>> {
        if let Some(dispute_state) = self.clone().disupte_state(root_tournament).await {
            // TODO: terminate all involved cartesi vm processes            
            let disputes = self.clone().disputes.clone();
            disputes.lock().await.get_mut(&root_tournament).unwrap().state.finished = true;
            Ok(dispute_state.clone())
        } else {
            Err(Box::new(EngineError::DsiputeNotFound(root_tournament.to_string())))
        }
    }

    pub async fn disupte_state(
        self: Arc<Self>,
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
        self: Arc<Self>,
        arena: Arc<A>,
        adversary: bool,
        root_tournament: Address,
    ) -> Result<(), Box<dyn Error>> {
       let dispute = if let Some(dispute) = self.clone().disupte_state(root_tournament).await {
            dispute    
       } else {
            return Err(Box::new(EngineError::DsiputeNotFound(root_tournament.to_string())))
       };
       
       let (machine, commitment_builder) = self.clone().create_player_machine(&dispute.machine_snapshot_path, adversary).await?;
       {
            let disputes = self.disputes.clone();
            let mut disputes = disputes.lock().await;
            if let Some(dispute) = disputes.get_mut(&root_tournament) {
                let player = Player::new(
                    arena,
                    machine,
                    commitment_builder,
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
        self: Arc<Self>,
        snapshot_path: &String,
        fake: bool,
    ) -> Result<(Arc<Mutex<MachineRpc>>, Arc<Mutex<dyn MachineCommitmentBuilder + Send>>), Box<dyn Error>> {
        let factory_lock = self.machine_factory.clone();
        let mut factory = factory_lock.lock().await;
        let snapshot_path = Path::new(snapshot_path);
        let machine = factory.create_machine(snapshot_path).await?;

        let commitment_builder: Arc<Mutex<dyn MachineCommitmentBuilder + Send>> = if fake {
            // TODO: pass parameters here or add them to config
            Arc::new(Mutex::new(FakeMachineCommitmentBuilder::new(Hash::default(), Some(Hash::default()))))
        } else {
            Arc::new(Mutex::new(CachingMachineCommitmentBuilder::new(machine.clone())))
        };

        Ok((machine, commitment_builder))
    }
}
