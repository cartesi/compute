use std::error::Error;

use async_trait::async_trait;
use primitive_types::H160;

use crate::{
    merkle::{Hash, MerkleProof, join_merkle_tree_node_digests},
    machine::MachineProof,
};

pub type Address = H160;

#[async_trait]
pub trait Arena : Send + Sync {
    async fn create_root_tournament(
        &self,
        initial_hash: Hash
    ) -> Result<Address, Box<dyn Error>>; 

    async fn join_tournament(
        &self,
        tournament: Address, 
        final_state: Hash,
        proof: MerkleProof,
        left_child: Hash,
        right_child: Hash
    ) -> Result<(), Box<dyn Error>>;
    
    async fn advance_match(
        &self,
        tournament: Address, 
        match_id: MatchID, 
        left_node: Hash,
        right_node: Hash,
        new_left_node: Hash,
        new_right_node:Hash
    ) -> Result<(), Box<dyn Error>>;
    
    async fn seal_inner_match(
        &self,
        tournament: Address,
        match_id: MatchID,
        left_leaf: Hash,
        right_leaf: Hash,
        initial_hash: Hash,
        initial_hash_proof: MerkleProof,
    ) -> Result<(), Box<dyn Error>>;
    
    async fn win_inner_match(
        &self,
        tournament: Address,
        child_tournament: Address,
        left_node: Hash,
        right_node: Hash,
    ) -> Result<(), Box<dyn Error>>;
    
    async fn seal_leaf_match(
        &self,
        tournament: Address,
        match_id: MatchID,
        left_leaf: Hash,
        right_leaf: Hash,
        initial_hash: Hash,
        initial_hash_proof: MerkleProof,
    ) -> Result<(), Box<dyn Error>>;
    
    async fn win_leaf_match(
        &self,
        tournament: Address,
        match_id: MatchID,
        left_node: Hash,
        right_node: Hash,
        proofs: MachineProof,
    ) -> Result<(), Box<dyn Error>>;

    async fn created_tournament(
        &self,
        tournament: Address,
        match_id: MatchID,   
    ) -> Result<Option<TournamentCreatedEvent>, Box<dyn Error>>;
    
    async fn created_matches(
        &self,
        tournament: Address,
        commitment_hash: Hash,
    ) -> Result<Vec<MatchCreatedEvent>, Box<dyn Error>>;
   
    async fn commitment(
        &self,
        tournament: Address,
        commitment_hash: Hash
    ) -> Result<(ClockState, Hash), Box<dyn Error>>;
    
    async fn match_state(
        &self,
        tournament: Address,
        match_id: MatchID,
    )-> Result<Option<MatchState>, Box<dyn Error>>;

    async fn root_tournament_winner(
        &self,
        tournament: Address
    ) -> Result<Option<(Hash, Hash)>, Box<dyn Error>>;
    
    async fn tournament_winner(
        &self,
        tournament: Address
    )-> Result<Option<Hash>, Box<dyn Error>>;
    
    async fn maximum_delay(
        &self,
        tournament: Address
    ) -> Result<u64, Box<dyn Error>>;
}

#[derive(Clone, Copy)]

pub struct TournamentCreatedEvent {
    pub parent_match_id_hash: Hash,
    pub new_tournament_address: Address,
}

#[derive(Clone, Copy)]

pub struct MatchCreatedEvent {
    pub id: MatchID,
    pub left_hash: Hash,    
}

#[derive(Clone, Copy)]
pub struct MatchID {
    pub commitment_one: Hash,
    pub commitment_two: Hash,
}

impl MatchID {
    pub fn hash(&self) -> Hash {
        join_merkle_tree_node_digests(self.commitment_one, self.commitment_two)
    }
}

#[derive(Clone, Copy)]
pub struct ClockState {
    pub allowance: u64,
    pub start_instant: u64,
}

#[derive(Clone, Copy)]
pub struct MatchState {
    pub other_parent: Hash,
    pub left_node: Hash,
    pub right_node: Hash,
    pub running_leaf_position: u64,
    pub current_height: u64,
    pub level: u64,
}