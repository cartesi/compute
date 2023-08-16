use std::{
    error::Error,
    collections::HashMap,
};

use::log::info;

use crate::{
    arena::{Arena, Address, Hash, Proof, MatchState, MatchCreatedEvent},
    machine::{ComputationCommitment, Machine},
};

static LEVELS: u64 = 4;
static LOG2_STEP: [u64; 4] = [24, 14, 7, 0];
static LOG2_UARCH_SPAN: u64 = 16;
static HEIGHTS: [u64; 4] = [39, 10, 7, 7];


#[derive(Debug)]
pub enum PlayerTournamentResult {
    TournamentWon,
    TournamentLost,
}

#[derive(Debug)]
struct PlayerTournament {
    address: Address,
    level: u64,
    parent: Option<Address>,
    base_big_cycle: u128,
}

#[derive(Debug)]
struct PlayerMatch {
    state: MatchState,
    event: MatchCreatedEvent,
    tournament: Address,
    leaf_cycle: u128,
}

pub struct Player {
    arena: Box<dyn Arena>,
    machine: Box<dyn Machine>,
    root_tournament: Address,
    tournaments: HashMap<Address, PlayerTournament>,
    matches: HashMap<Hash, PlayerMatch>,
    commitments: HashMap<Address, Box<ComputationCommitment>>,
    called_win: HashMap<Address, bool>,
}

impl Player {
    pub fn new(arena: Box<dyn Arena>, machine: Box<dyn Machine>, root_tournamet: Address) -> Self {
        Self {
            arena: arena,
            machine: machine,
            root_tournament: root_tournamet,
            tournaments: HashMap::from([
                (root_tournamet, PlayerTournament{
                    address: root_tournamet,
                    level: LEVELS,
                    parent: None,
                    base_big_cycle: 0,
                })
            ]),
            matches: HashMap::<Hash, PlayerMatch>::new(),
            commitments: HashMap::<Address, Box<ComputationCommitment>>::new(),
            called_win: HashMap::<Address, bool>::new(),
        }
    }

    pub async fn react(&mut self) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        self.react_tournament(self.root_tournament).await
    }

    async fn react_tournament(
        &mut self,
        tournament: Address,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let tournament = self.tournaments.get(&tournament).unwrap();        
        
        let commitment = if let Some(commitment) = self.commitments.get(&tournament.address) {
            commitment
        } else {
            let commitment = self.machine.build_commitment(
                LOG2_STEP[tournament.level as usize],
                HEIGHTS[(LEVELS - tournament.level) as usize],
                false, 
                false,
            ).await?;
            self.commitments.insert(tournament.address, Box::new(commitment));
            self.commitments.get(&tournament.address).unwrap()
        };

        if tournament.parent.is_none() {
            if let Some(winner) = self.arena.root_tournament_winner(tournament.address).await? {
                info!("tournament {} finished, winner is {}", tournament.address, winner);
                if commitment.root == winner {
                    return Ok(Some(PlayerTournamentResult::TournamentWon));
                } else {
                    return Ok(Some(PlayerTournamentResult::TournamentLost));
                }
            } else {
                return Ok(None);
            }
        } else {
            if let Some(winner) = self.arena.tournament_winner(tournament.address).await? {
                let old_commitment = self.commitments.get(&tournament.parent.unwrap()).unwrap();
                if winner != old_commitment.root {
                    return Ok(Some(PlayerTournamentResult::TournamentLost));
                }
            }

            if self.called_win.contains_key(&tournament.address) {
                return Ok(None);
            } else {
                self.called_win.insert(tournament.address, true);
            }

            info!(
                "player won tournament {} of level {} for commitment {}",
                tournament.address,
                tournament.level,
                commitment.root,
            );
        }

        match self.latest_match(tournament.address, commitment).await? {
            Some(latest_match) => self.react_match(latest_match, commitment).await,
            None => {
                self.join_tournament_if_needed(tournament.address, commitment).await?;
                Ok(None)
            }
        }
    }

    async fn latest_match(
        &mut self,
        tournament: Address,
        commitment: &ComputationCommitment,
    ) -> Result<Option<Hash>, Box<dyn Error>> {
        let matches = self.arena.created_matches(tournament, commitment.root).await?;
        let last_match = if let Some(last_match) = matches.last() {
            last_match
        } else {
            return Ok(None)
        };

        let match_state = if let Some(m) = self.arena.match_state(
            tournament, 
            last_match.id,
        ).await? {
            m
        } else {
            return Ok(None)
        };

        let match_id_hash = last_match.id.hash();
        let player_tournament = self.tournaments.get_mut(&tournament).unwrap();
        self.matches.insert(match_id_hash, PlayerMatch {
            state: match_state,
            event: *last_match,
            tournament: tournament,
            leaf_cycle: player_tournament.base_big_cycle  + 
                (match_state.running_leaf_position << (LOG2_STEP[player_tournament.level as usize] + LOG2_UARCH_SPAN)),
        });

        Ok(Some(match_id_hash))
    }

    async fn join_tournament_if_needed(
        &mut self,
        tournament: Address,
        commitment: &ComputationCommitment,
    ) -> Result<(), Box<dyn Error>> {
        let tournament = self.tournaments.get(&tournament).unwrap();
        
        let (clock, _) = self.arena.commitment(
            tournament.address,
            commitment.root,
        ).await?;

        if clock.allowance != 0 {
            return Ok(())
        }

        let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root) {
            children
        } else {
            // TODO: return error
            panic!("node children not found")
        };

        let (last, proof) = commitment.last();
        self.arena.join_tournament(
            tournament.address,
            last, proof,
            left_child,
            right_child
        ).await
    } 

    async fn react_match(
        &mut self,
        match_id_hash: Hash,
        commitment: &ComputationCommitment,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let player_match = self.matches.get(&match_id_hash).unwrap();
        if player_match.state.current_height == 0 {
            self.react_sealed_match(match_id_hash, commitment).await
        } else if player_match.state.current_height == 1 {
            self.react_unsealed_match(match_id_hash, commitment).await
        } else {
            self.react_running_match(match_id_hash, commitment).await
        }
    } 

    async fn react_sealed_match(
        &mut self,
        match_id_hash: Hash,
        commitment: &ComputationCommitment,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let player_match = self.matches.get(&match_id_hash).unwrap(); 
        if player_match.state.level == 1 {
            let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root) {
                children
            } else {
                // TODO: return error
                panic!("node children not found")
            };
            
            if let Some(_) = self.arena.match_state(
                player_match.tournament,
                player_match.event.id,
            ).await? {
                let delay = self.arena.maximum_delay(player_match.tournament).await?;
                info!("delay for match {} is {}", player_match.event.id.hash(), delay);
                return Ok(None)
            }
            
            self.arena.win_leaf_match(
                player_match.tournament,
                player_match.event.id,
                left_child,
                right_child,
            ).await?;
            return Ok(None)
        } else {
            let new_tournament = self.new_tournament(match_id_hash).await?;
            self.react_tournament(new_tournament).await
        }
    }

    async fn react_unsealed_match(
        &mut self,
        match_id_hash: Hash,
        commitment: &ComputationCommitment,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {        
        let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root) {
            children
        } else {
            return Ok(None)
        };

        let player_match = self.matches.get(&match_id_hash).unwrap();
        let (initial_hash, initial_hash_proof) = if player_match.state.running_leaf_position == 0 {
            (self.machine.initial_hash().await?, Proof::default())
        } else {
            commitment.prove_leaf(player_match.state.running_leaf_position)
        };

        let tournament = self.tournaments.get(&player_match.tournament).unwrap();
        if tournament.level == 1 {
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left_child,
                right_child,
                initial_hash,
                initial_hash_proof
            ).await?;
            return Ok(None)
        } else {
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left_child,
                right_child,
                initial_hash,
                initial_hash_proof
            ).await?;
            
            let new_tournament = self.new_tournament(match_id_hash).await?;
            self.react_tournament(new_tournament).await
        }
    }

    async fn react_running_match(
        &mut self,
        match_id_hash: Hash,
        commitment: &ComputationCommitment,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root) {
            children
        } else {
            return Ok(None)
        };

        let player_match = self.matches.get(&match_id_hash).unwrap();
        let parent = if left_child != player_match.state.left_node {
            left_child
        } else {
            right_child
        };
        let (new_left, new_right) = if let Some(children) = commitment.chidlren(parent) {
            children
        } else {
             // TODO: return error
             panic!("node children not found")
        };

        self.arena.advance_match(
            player_match.tournament,
            player_match.event.id,
            left_child,
            right_child,
            new_left,
            new_right,
        ).await?;

        Ok(None)
    }

    async fn new_tournament(
        &mut self,
        match_id_hash: Hash,
    ) -> Result<Address, Box<dyn Error>> {
        let player_match = self.matches.get(&match_id_hash).unwrap();
        
        let address = self.arena.created_tournament(
            player_match.tournament,
            player_match.event.id,
        ).await?.unwrap().new_tournament_address;

        self.tournaments.insert(address, PlayerTournament {
            address: address,
            level: player_match.state.level - 1,
            parent: Some(player_match.tournament),
            base_big_cycle: player_match.leaf_cycle,
        });

        Ok(address)
    }
}