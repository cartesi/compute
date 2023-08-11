use std::error::Error;

use async_trait::async_trait;
use primitive_types::H160;

#[async_trait]
pub trait Arena : Send + Sync {
    async fn init(&mut self) -> Result<(), Box<dyn Error>>;
    
    async fn create_root_tournament(&mut self, initial_hash: Hash) -> Result<Address, Box<dyn Error>>;

    async fn join_tournament(
        &mut self,
        tournament: Address, 
        final_state: Hash,
        proof: Proof,
        left_child: Hash,
        right_child: Hash
    ) -> Result<(), Box<dyn Error>>;
    
    async fn advance_match(
        &mut self,
        tournament: Address, 
        match_id: MatchID, 
        left_node: Hash,
        right_node: Hash,
        new_left_node: Hash,
        new_right_node:Hash
    ) -> Result<(), Box<dyn Error>>;
    
    async fn seal_inner_match(
        &mut self,
        tournament: Address,
        match_id: MatchID,
        left_leaf: Hash,
        right_leaf: Hash,
        initial_hash: Hash,
        initial_hash_proof: Proof
    ) -> Result<(), Box<dyn Error>>;
    
    async fn win_inner_match(
        &mut self,
        tournament: Address,
        child_tournament: Address,
        left_node: Hash,
        right_node: Hash,
    ) -> Result<(), Box<dyn Error>>;
    
    async fn seal_leaf_match(
        &mut self,
        tournament: Address,
        match_id: MatchID,
        left_leaf: Hash,
        right_leaf: Hash,
        initial_hash: Hash,
        initial_hash_proof: Proof,
    ) -> Result<(), Box<dyn Error>>;
    
    async fn win_leaf_match(
        &mut self,
        tournament: Address,
        match_id: MatchID,
        left_node: Hash,
        right_node: Hash
    ) -> Result<(), Box<dyn Error>>;

    async fn created_tournament(
        &self,
        tournament: Address,
        match_id_hash: Hash,   
    ) -> Result<TournamentCreatedEvent, Box<dyn Error>>;
    
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
        match_id_hash: Hash
    )-> Result<Option<MatchState>, Box<dyn Error>>;

    async fn root_tournament_winner(
        &self,
        tournament: Address
    ) -> Result<Option<Hash>, Box<dyn Error>>;
    
    async fn tournament_winner(
        &self,
        tournament: Address
    )-> Result<Option<Hash>, Box<dyn Error>>;
    
    async fn maximum_delay(
        &self,
        tournament: Address
    ) -> Result<u64, Box<dyn Error>>;
}

#[derive(Debug, Clone, Copy)]

pub struct TournamentCreatedEvent {
    pub parent_match_id_hash: Hash,
    pub address: Address,
}

#[derive(Debug, Clone, Copy)]

pub struct MatchCreatedEvent {
    pub commitment_one: Hash,
    pub commitment_two: Hash,
    pub left_hash: Hash,
    pub id_hash: Hash,    
}

#[derive(Debug, Clone, Copy)]
pub struct ClockState {
    pub allowance: u64,
    pub start_instant: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct MatchState {
    pub other_parent: Hash,
    pub left_node: Hash,
    pub right_node: Hash,
    pub running_leaf_position: u64,
    pub current_height: u64,
    pub level: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct MatchID {
    pub commitment_one: Hash,
    pub commitment_two: Hash,
}

pub type Address = H160;
pub type Hash = [u8; 32];
pub type Proof = Vec<Hash>;

pub fn is_hash_zero(hash: Hash) -> bool {
    // TODO
    return false;
}