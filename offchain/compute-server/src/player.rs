use std::{
    error::Error,
    collections::HashMap,
    sync::Arc,
};

use::tokio::sync::Mutex;

use::log::info;

use crate::{
    arena::{Arena, Address, MatchState, MatchCreatedEvent},
    merkle::MerkleProof,
    machine::{constants, MachineRpc, MachineCommitment, MachineCommitmentBuilder},
};

// !!!
/*
static LEVELS: u64 = 4;
static LOG2_STEP: [u64; 4] = [24, 14, 7, 0];
static LOG2_UARCH_SPAN: u64 = 16;
static UARCH_SPAN: u128 =  (1 << LOG2_UARCH_SPAN) - 1;
static HEIGHTS: [u64; 4] = [39, 10, 7, 7];
*/

pub enum PlayerTournamentResult {
    TournamentWon,
    TournamentLost,
}

struct PlayerTournament {
    address: Address,
    level: u64,
    parent: Option<Address>,
    base_big_cycle: u128,
}

struct PlayerMatch {
    state: MatchState,
    event: MatchCreatedEvent,
    tournament: Address,
    leaf_cycle: u128,
    base_big_cycle: u128,
}

// TODO: use tempaltes, not box
pub struct Player<A: Arena> {
    arena: Arc<A>,
    machine: Arc<MachineRpc>,
    commitment_builder: Arc<Mutex<dyn MachineCommitmentBuilder + Send>>,
    tournaments: Vec<Arc<PlayerTournament>>,
    matches: Vec<Arc<PlayerMatch>>,
    commitments: HashMap<Address, Arc<MachineCommitment>>,
    called_win: HashMap<Address, bool>,
}

impl<A: Arena> Player<A> {
    pub fn new(
        arena: Arc<A>,
        machine: Arc<MachineRpc>,
        commitment_builder: Arc<Mutex<dyn MachineCommitmentBuilder + Send>>,
        root_tournamet: Address
    ) -> Self {
        Player {
            arena: arena,
            machine: machine,
            commitment_builder: commitment_builder,
            tournaments: vec![
                Arc::new(PlayerTournament{
                    address: root_tournamet,
                    level: constants::LEVELS,
                    parent: None,
                    base_big_cycle: 0,
                }),
            ],
            matches: Vec::new(),
            commitments: HashMap::new(),
            called_win: HashMap::new(),
        }
    }

    pub async fn react(&mut self) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let last_tournament = self.tournaments.last().unwrap().clone();
        self.react_tournament(last_tournament).await
    }

    async fn react_tournament(
        &mut self,
        tournament: Arc<PlayerTournament>,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let commitment = if let Some(commitment) = self.commitments.get(&tournament.address) {
            commitment.clone()
        } else {
            let commitment = self.commitment_builder.clone().lock().await.build_commitment(
                constants::LOG2_STEP[tournament.level as usize],
                constants::HEIGHTS[(constants::LEVELS - tournament.level) as usize],
            ).await?;
            self.commitments.insert(tournament.address, Arc::new(commitment));
            self.commitments.get(&tournament.address).unwrap().clone()
        };

        if tournament.parent.is_none() {
            if let Some((winner_commitment, _)) = self.arena.root_tournament_winner(tournament.address).await? {
                info!("tournament {} finished, winner is {}", tournament.address, winner_commitment);
                if commitment.merkle.root_hash() == winner_commitment {
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
                if winner != old_commitment.merkle.root_hash() {
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
                commitment.merkle.root_hash(),
            );
        }

        match self.latest_match(tournament.address, commitment.clone()).await? {
            Some(latest_match) => self.react_match(latest_match, commitment).await?,
            None => self.join_tournament_if_needed(tournament, commitment).await?,
        }

        Ok(None)
    }

    async fn latest_match(
        &mut self,
        tournament: Address,
        commitment: Arc<MachineCommitment>,
    ) -> Result<Option<Arc<PlayerMatch>>, Box<dyn Error>> {
        let matches = self.arena.created_matches(tournament, commitment.merkle.root_hash()).await?;
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

        let tournament = self.tournaments.iter().find(|t| t.address == tournament).unwrap();
        let base = tournament.base_big_cycle;
        let step = 1 << constants::LOG2_STEP[tournament.level as usize];
        let leaf_cycle = base + (step * match_state.running_leaf_position);
        let base_big_cycle = leaf_cycle >> constants::LOG2_UARCH_SPAN;
        
        let player_match = Arc::new(PlayerMatch {
            state: match_state,
            event: *last_match,
            tournament: tournament.address,
            leaf_cycle: leaf_cycle,
            base_big_cycle: base_big_cycle,
        });
        self.matches.push(player_match.clone());

        Ok(Some(player_match))
    }

    async fn join_tournament_if_needed(
        &mut self,
        tournament: Arc<PlayerTournament>,
        commitment: Arc<MachineCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        let (clock, _) = self.arena.commitment(
            tournament.address,
            commitment.merkle.root_hash(),
        ).await?;

        if clock.allowance != 0 {
            return Ok(())
        }

        let (left, right) = commitment.merkle.root_children().expect("root does not have children");
        let (last, proof) = commitment.merkle.last();

        self.arena.join_tournament(
            tournament.address,
            last,
            proof,
            left.digest,
            right.digest,
        ).await
    } 

    async fn react_match(
        &mut self,
        player_match: Arc<PlayerMatch>,
        commitment: Arc<MachineCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        if player_match.state.current_height == 0 {
            self.react_sealed_match(player_match, commitment).await
        } else if player_match.state.current_height == 1 {
            self.react_unsealed_match(player_match, commitment).await
        } else {
            self.react_running_match(player_match, commitment).await
        }
    } 

    async fn react_sealed_match(
        &mut self,
        player_match: Arc<PlayerMatch>,
        commitment: Arc<MachineCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        if player_match.state.level == 1 {
            let (left, right) = commitment.merkle.root_children().expect("root does not have chidlren");
            
            // Probably, player_match.state.other_parentca be used here.
            let match_state = self.arena
                .match_state(player_match.tournament, player_match.event.id)
                .await?
                .expect("match not found");
            let finished = player_match.state.other_parent.is_zero();
            if finished {
                return Ok(())
            }
            
            if let Some(_) = self.arena.match_state(
                player_match.tournament,
                player_match.event.id,
            ).await? {
                let delay = self.arena.maximum_delay(player_match.tournament).await?;
                info!("delay for match {} is {}", player_match.event.id.hash(), delay);
                return Ok(())
            }

            let cycle = player_match.state.running_leaf_position >> constants::LOG2_UARCH_SPAN;
            let ucycle = player_match.state.running_leaf_position & constants::UARCH_SPAN;
            let proof = self.machine.generate_proof(cycle as u64, ucycle as u64).await?;
            
            self.arena.win_leaf_match(
                player_match.tournament,
                player_match.event.id,
                left.digest,
                right.digest,
                proof,
            ).await?;
        } else {
            self.new_tournament(player_match).await?;
        }
        
        Ok(())
    }

    async fn react_unsealed_match(
        &mut self,
        player_match: Arc<PlayerMatch>,
        commitment: Arc<MachineCommitment>,
    ) -> Result<(), Box<dyn Error>> {        
        let current_other_parent = commitment.merkle
            .node(player_match.state.other_parent)
            .expect("failed to find merkle tree node");
        let (left, right) = if let Some(children) = current_other_parent.children() {
            children
        } else {
            return Ok(())
        };

        let (initial_hash, initial_hash_proof) = if player_match.state.running_leaf_position == 0 {
            (commitment.implicit_hash, MerkleProof::default())
        } else {
            commitment.merkle.prove_leaf(player_match.state.running_leaf_position)
        };

        let tournament = self.tournaments.iter().find(|t| t.address == player_match.tournament).unwrap();
        if tournament.level == 1 {
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left.digest,
                right.digest,
                initial_hash,
                initial_hash_proof
            ).await?;
        } else {
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left.digest,
                right.digest,
                initial_hash,
                initial_hash_proof
            ).await?;
            self.new_tournament(player_match).await?;
        }

        Ok(())
    }

    async fn react_running_match(
        &mut self,
        player_match: Arc<PlayerMatch>,
        commitment: Arc<MachineCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        let current_other_parent = commitment.merkle
            .node(player_match.state.other_parent)
            .expect("failed to find merkle tree node");
        let (left, right) = if let Some(children) = current_other_parent.children() {
            children
        } else {
            return Ok(())
        };

        let (new_left, new_right) = if left.digest != player_match.state.left_node {
            left.children().expect("let node does not have children")
        } else {
            right.children().expect("right node does not have children")
        };

        self.arena.advance_match(
            player_match.tournament,
            player_match.event.id,
            left.digest,
            right.digest,
            new_left.digest,
            new_right.digest,
        ).await?;

        Ok(())
    }

    async fn new_tournament(
        &mut self,
        player_match: Arc<PlayerMatch>,
    ) -> Result<(), Box<dyn Error>> {
        let address = self.arena.created_tournament(
            player_match.tournament,
            player_match.event.id,
        ).await?.unwrap().new_tournament_address;

        let tournament = Arc::new(PlayerTournament {
            address: address,
            level: player_match.state.level - 1,
            parent: Some(player_match.tournament),
            base_big_cycle: player_match.leaf_cycle,
        });
        self.tournaments.push(tournament);

        Ok(())
    }
}