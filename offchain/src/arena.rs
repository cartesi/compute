use std::error::Error;

use async_trait::async_trait;
use primitive_types::H160;

#[async_trait]
pub trait Arena {
    async fn create_root_tournament(&mut self, initial_hash: Hash);
    
    async fn join_tournament(
        &mut self,
        tournament_address: Address, 
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
        tournamet: Address,
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
        right_node: Hash,
    ) -> Result<(), Box<dyn Error>>;

    async fn created_tournaments(&self) -> Result<Vec<TournamentInfo>, Box<dyn Error>>;
    async fn created_matches(&self) -> Result<Vec<MatchInfo>, Box<dyn Error>>;
   
    async fn commitment(
        &self,
        tournament: Address,
        commitment_hash: Hash
    ) -> Result<(ClockState, Hash), Box<dyn Error>>;
    
    async fn match_state(
        &self,
        match_address: Address,
        match_id_hash: Hash
    )-> Result<MatchState, Box<dyn Error>>;

    async fn root_tournament_winner(&self, tournamet: Address) -> Result<Hash, Box<dyn Error>>;
    async fn tournament_winner(&self, tournament: Address) -> Result<Hash, Box<dyn Error>>;
    
    async fn maximum_delay(&self, tournament: Address) -> Result<u64, Box<dyn Error>>;
}

pub struct TournamentInfo {

}

pub struct MatchInfo {

}

pub struct ClockState {
    pub allowance: u64,
    pub start_instant: u64,
}

pub struct MatchState {
    pub other_parent: Hash,
    pub left_node: Hash,
    pub right_node: Hash,
    pub running_leaf_position: u64,
    pub height: u64,
    pub current_height: u64,
}

pub struct MatchID {
    commitment_one: Hash,
    commitment_two: Hash,
}

type Address = H160;
type Hash = [u8; 32];
type Proof = Vec<Hash>;