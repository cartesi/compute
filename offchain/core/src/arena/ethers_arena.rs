use std::{
    error::Error,
    sync::Arc,
    path::Path,
    time::Duration, str::FromStr,
};

use async_trait::async_trait;

use ethers::{
    core::abi::Tokenize,
    types::{
        Filter,
        Address as EthersAddress,
        Bytes,
    },
    contract::ContractFactory,
    providers::{Provider, Http, Middleware},
    signers::{LocalWallet, Signer},
    middleware::SignerMiddleware,
};

use crate::{
    contract::{
        factory::TournamentFactory,
        tournament::{
            non_root_tournament,
            root_tournament,
            non_leaf_tournament,
            leaf_tournament,
            tournament,
        },
    },
    arena::*,
    merkle::{Hash, MerkleProof},
    machine::MachineProof,
};

pub struct EthersArena {
    // Start anvil for testing.
    config: ArenaConfig,
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    tournament_factory: EthersAddress,
}

impl EthersArena {
    pub fn new(config: ArenaConfig) -> Result<Self, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(config.web3_rpc_url.clone())?
            .interval(Duration::from_millis(10u64));
        let wallet = LocalWallet::from_str(config.web3_private_key.as_str())?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(config.web3_chain_id)));
        
        Ok(EthersArena {
            config,
            client,
            tournament_factory: EthersAddress::default(),
        })
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        // Deploy single level factory.
        let sl_factory_artifact = Path::new(self.config.contract_artifacts.single_level_factory.as_str());
        let sl_factory_address = self.deploy_contract_from_artifact(sl_factory_artifact, ()).await?;

        // Deploy top factory.
        let top_factory_artifact = Path::new(self.config.contract_artifacts.top_factory.as_str());
        let top_factory_address = self.deploy_contract_from_artifact(top_factory_artifact, ()).await?;

        // Deploy middle factory.
        let middle_factory_artifact = Path::new(self.config.contract_artifacts.middle_factory.as_str());
        let middle_factory_address = self.deploy_contract_from_artifact(middle_factory_artifact, ()).await?;

        // Deploy bottom factory.
        let bottom_factory_artifact = Path::new(self.config.contract_artifacts.bottom_factory.as_str());
        let bottom_factory_address = self.deploy_contract_from_artifact(bottom_factory_artifact, ()).await?;

        // Deploy tournament factory.
        let tournament_factory_artifact = Path::new(self.config.contract_artifacts.tournament_factory.as_str());
        self.tournament_factory = self.deploy_contract_from_artifact(
            tournament_factory_artifact, 
            (sl_factory_address, top_factory_address, middle_factory_address, bottom_factory_address),
        ).await?;
        
        return Ok(());
    }

    async fn deploy_contract_from_artifact<T: Tokenize>(
        &self, 
        artifact_path: &Path, 
        constuctor_args: T
    ) -> Result<EthersAddress, Box<dyn Error>> {
        let (abi, bytecode) = parse_artifact(artifact_path)?;
        let deployer = ContractFactory::new(abi, bytecode, self.client.clone());
        let contract = deployer
            .deploy(constuctor_args)?
            .confirmations(0usize)
            .send()
            .await?;
        Ok(contract.address())
    }
}

#[async_trait]
impl Arena for EthersArena {
    async fn create_root_tournament(
        &self,
        initial_hash: Hash
    ) -> Result<Address, Box<dyn Error>> {
        let factory = TournamentFactory::new(self.tournament_factory, self.client.clone());
        factory.instantiate_top(initial_hash.into())
            .send()
            .await?
            .await?;
     
        let filter = Filter::new()
            .from_block(0);
        let logs = self.client.get_logs(&filter).await?;

        // !!!
        println!("{}", logs.len());

        Ok(Address::default())
    }
    
    async fn join_tournament(
        &self,
        tournament: Address, 
        final_state: Hash,
        proof: MerkleProof,
        left_child: Hash,
        right_child: Hash
    ) -> Result<(), Box<dyn Error>> {
        let tournament = tournament::Tournament::new(tournament, self.client.clone());
        let proof = proof.iter().map(|h| -> [u8;32] { h.clone().into() }).collect();
        tournament
            .join_tournament(final_state.into(), proof, left_child.into(), right_child.into())
            .send()
            .await?
            .await?;
        Ok(())
    }
    
    async fn advance_match(
        &self,
        tournament: Address, 
        match_id: MatchID, 
        left_node: Hash,
        right_node: Hash,
        new_left_node: Hash,
        new_right_node:Hash
    ) -> Result<(), Box<dyn Error>> {
        let tournament = tournament::Tournament::new(tournament, self.client.clone());
        let match_id = tournament::Id { 
            commitment_one: match_id.commitment_one.into(),
            commitment_two: match_id.commitment_two.into()
        };
        tournament.
            advance_match(
                match_id,
                left_node.into(),
                right_node.into(),
                new_left_node.into(),
                new_right_node.into())
            .send()
            .await?
            .await?;
        Ok(())
    }
    
    async fn seal_inner_match(
        &self,
        tournament: Address,
        match_id: MatchID,
        left_leaf: Hash,
        right_leaf: Hash,
        initial_hash: Hash,
        initial_hash_proof: MerkleProof
    ) -> Result<(), Box<dyn Error>> {
        let tournament = non_leaf_tournament::NonLeafTournament::new(tournament, self.client.clone());
        let match_id = non_leaf_tournament::Id {
            commitment_one: match_id.commitment_one.into(),
            commitment_two: match_id.commitment_two.into(),
        };
        let initial_hash_proof = initial_hash_proof.iter().map(|h| -> [u8;32] { h.clone().into() }).collect();
        tournament
            .seal_inner_match_and_create_inner_tournament(
                match_id,
                left_leaf.into(),
                right_leaf.into(),
                initial_hash.into(),
                initial_hash_proof
            )
            .send()
            .await?
            .await?;
        Ok(())
    }
    
    async fn win_inner_match(
        &self,
        tournament: Address,
        child_tournament: Address,
        left_node: Hash,
        right_node: Hash,
    ) -> Result<(), Box<dyn Error>> {
        let tournament = non_leaf_tournament::NonLeafTournament::new(tournament, self.client.clone());
        tournament.win_inner_match(child_tournament, left_node.into(), right_node.into())
            .send()
            .await?
            .await?;
        Ok(())
    }
    
    async fn seal_leaf_match(
        &self,
        tournament: Address,
        match_id: MatchID,
        left_leaf: Hash,
        right_leaf: Hash,
        initial_hash: Hash,
        initial_hash_proof: MerkleProof,
    ) -> Result<(), Box<dyn Error>> {
        let tournament = leaf_tournament::LeafTournament::new(tournament, self.client.clone());
        let match_id = leaf_tournament::Id {
            commitment_one: match_id.commitment_one.into(),
            commitment_two: match_id.commitment_two.into()
        };
        let initial_hash_proof = initial_hash_proof.iter().map(|h| -> [u8;32] { h.clone().into() }).collect();
        tournament
            .seal_leaf_match(
                match_id,
                left_leaf.into(),
                right_leaf.into(),
                initial_hash.into(),
                initial_hash_proof
            )
            .send()
            .await?
            .await?;
        Ok(())
    }
    
    async fn win_leaf_match(
        &self,
        tournament: Address,
        match_id: MatchID,
        left_node: Hash,
        right_node: Hash,
        proofs: MachineProof,
    ) -> Result<(), Box<dyn Error>> {
        let tournament = leaf_tournament::LeafTournament::new(tournament, self.client.clone());
        let match_id = leaf_tournament::Id {
            commitment_one: match_id.commitment_one.into(),
            commitment_two: match_id.commitment_two.into(),
        };
        tournament
            .win_leaf_match(
                match_id,
                left_node.into(),
                right_node.into(),
                Bytes::from(proofs),
            )
            .send()
            .await?
            .await?;
        Ok(())
    }

    async fn created_tournament(
        &self,
        tournament: Address,
        match_id: MatchID,           
    ) -> Result<Option<TournamentCreatedEvent>, Box<dyn Error>> {
        let tournament = non_leaf_tournament::NonLeafTournament::new(tournament, self.client.clone());
        let events = tournament.new_inner_tournament_filter().query().await.unwrap();
        if let Some(event) = events.first() {
            Ok(Some(TournamentCreatedEvent {
                parent_match_id_hash: match_id.hash(),
                new_tournament_address: event.1,
            }))
        } else {
            Ok(None)
        }
    }
    
    async fn created_matches(
        &self,
        tournament: Address,
        commitment_hash: Hash,
    ) -> Result<Vec<MatchCreatedEvent>, Box<dyn Error>> {
        let tournament = tournament::Tournament::new(tournament, self.client.clone());
        let events = tournament.match_created_filter().query().await.unwrap();
        let events: Vec<MatchCreatedEvent> = events.iter().map(|event| {
            MatchCreatedEvent {
                id: MatchID {
                    commitment_one: event.two.into(),
                    commitment_two: event.left_of_two.into(),
                },
                left_hash: event.one.into(),
            }
        }).collect();
        Ok(events)
    }
   
    async fn commitment(
        &self,
        tournament: Address,
        commitment_hash: Hash,
    ) -> Result<(ClockState, Hash), Box<dyn Error>> {
        let tournament = tournament::Tournament::new(tournament, self.client.clone());
        let (clock_state, hash) = tournament.get_commitment(commitment_hash.into()).call().await?;
        let clock_state = ClockState {
            allowance: clock_state.allowance,
            start_instant: clock_state.start_instant
        };
        Ok((clock_state, Hash::from(hash)))
    }
    
    async fn match_state(
        &self,
        tournament: Address,
        match_id: MatchID,
    )-> Result<Option<MatchState>, Box<dyn Error>> {
        let tournament = tournament::Tournament::new(tournament, self.client.clone());
        let match_state = tournament.get_match(match_id.hash().into()).call().await?;
        if !Hash::from(match_state.other_parent).is_zero() {
            Ok(Some(MatchState { 
                other_parent: match_state.other_parent.into(),
                left_node: match_state.left_node.into(), 
                right_node: match_state.right_node.into(), 
                running_leaf_position: match_state.running_leaf_position.as_u64(), 
                current_height: match_state.current_height, 
                level: match_state.level,
            }))
        } else {
            Ok(None)
        }
    }

    async fn root_tournament_winner(
        &self,
        root_tournament: Address
    ) -> Result<Option<(Hash, Hash)>, Box<dyn Error>> {
        let root_tournament = root_tournament::RootTournament::new(root_tournament, self.client.clone());
        let (finished, commitment, state) = root_tournament.arbitration_result().call().await?;
        if finished {
            Ok(Some((
                Hash::from(commitment),
                Hash::from(state),
            )))
        } else {
            Ok(None)
        }
    }

    async fn tournament_winner(
        &self,
        tournament: Address
    ) -> Result<Option<Hash>, Box<dyn Error>> {
        let tournament = non_root_tournament::NonRootTournament::new(tournament, self.client.clone());
        let (finished, state) = tournament.inner_tournament_winner().call().await?;
        if finished {
            Ok(Some(Hash::from(state)))
        } else {
            Ok(None)
        }
    }
    
    async fn maximum_delay(&self, tournament: Address) -> Result<u64, Box<dyn Error>> {
        let tournament = tournament::Tournament::new(tournament, self.client.clone());
        let delay = tournament.maximum_enforceable_delay().call().await?;
        Ok(delay)
    } 
}