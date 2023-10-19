use std::{
    error::Error,
    collections::HashMap,
    sync::Arc,
};

use::tokio::sync::Mutex;

use::log::{info, debug};

use crate::{
    arena::{Arena, Address, MatchState, MatchCreatedEvent},
    merkle::MerkleProof,
    machine::{constants, MachineRpc, MachineCommitment, MachineCommitmentBuilder},
};

pub enum PlayerTournamentResult {
    TournamentWon,
    TournamentLost,
}

struct PlayerTournament {
    address: Address,
    level: u64,
    parent: Option<Address>,
    base_big_cycle: u64,
}

struct PlayerMatch {
    state: MatchState,
    event: MatchCreatedEvent,
    tournament: Address,
    leaf_cycle: u64,
    base_big_cycle: u64,
}

pub struct Player<A: Arena> {
    arena: Arc<A>,
    machine: Arc<Mutex<MachineRpc>>,
    commitment_builder: Arc<Mutex<dyn MachineCommitmentBuilder + Send>>,
    tournaments: Vec<Arc<PlayerTournament>>,
    matches: Vec<Arc<PlayerMatch>>,
    commitments: HashMap<Address, Arc<MachineCommitment>>,
    called_win: HashMap<Address, bool>,
}

impl<A: Arena> Player<A> {
    pub fn new(
        arena: Arc<A>,
        machine: Arc<Mutex<MachineRpc>>,
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
            if let Some((winner_commitment, winner_state)) = self.arena.root_tournament_winner(tournament.address).await? {
                info!(
                    "tournament {} finished - winner is {}, winner state hash is {}", 
                    tournament.address,
                    winner_commitment,
                    winner_state,
                );
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
                    info!("player lost tournament {}", tournament.address);
                    return Ok(Some(PlayerTournamentResult::TournamentLost));
                }

                if self.called_win.contains_key(&tournament.address) {
                    info!("player already called winInnerMatch for tournament {}", tournament.address);
                    return Ok(None);
                } else {
                    self.called_win.insert(tournament.address, true);
                }
    
                info!(
                    "win tournament {} of level {} for commitment {}",
                    tournament.address,
                    tournament.level,
                    commitment.merkle.root_hash(),
                );
                let (left, right) = old_commitment.merkle.root_children();
                self.arena.win_inner_match(
                    tournament.parent.unwrap(), 
                    tournament.address, 
                    left,
                    right,
                ).await?
            }
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

        let (left, right) = commitment.merkle.root_children();
        let (last, proof) = commitment.merkle.last();

        info!(
            "join tournament {} of level {} with commitment {}",
            tournament.address,
            tournament.level,
            commitment.merkle.root_hash(),
        );
        self.arena.join_tournament(
            tournament.address,
            last,
            proof,
            left,
            right,
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
        info!(
            "height for match {} is {}", 
            player_match.event.id.hash(),
            player_match.state.current_height
        );

        if player_match.state.level == 1 {
            let (left, right) = commitment.merkle.root_children();
            
            // Probably, player_match.state.other_parent can be used here.
            let match_state = self.arena
                .match_state(player_match.tournament, player_match.event.id)
                .await?
                .expect("match not found");
            let finished = match_state.other_parent.is_zero();
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
            let proof = self.machine
                .clone()
                .lock()
                .await
                .generate_proof(cycle, ucycle)
                .await?;
            
            info!(
                "win leaf match in tournament {} of level {} for commitment {}",
                player_match.tournament,
                self.tournament(player_match.tournament).level,
                commitment.merkle.root_hash(),
            );
            self.arena.win_leaf_match(
                player_match.tournament,
                player_match.event.id,
                left,
                right,
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
        let (left, right) = if let Some(children) = commitment.merkle.node_children(player_match.state.other_parent) {
            children
        } else {
            return Ok(())
        };

        let (initial_hash, initial_hash_proof) = if player_match.state.running_leaf_position == 0 {
            (commitment.implicit_hash, MerkleProof::default())
        } else {
            commitment.merkle.prove_leaf(player_match.state.running_leaf_position)
        };

        let tournament = self.tournament(player_match.tournament);
        if tournament.level == 1 {
            info!(
                "seal leaf match in tournament {} of level {} for commitment {}",
                player_match.tournament,
                tournament.level,
                commitment.merkle.root_hash(),
            );
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left,
                right,
                initial_hash,
                initial_hash_proof
            ).await?;
        } else {
            info!(
                "seal inner match in tournament {} of level {} for commitment {}",
                player_match.tournament,
                tournament.level,
                commitment.merkle.root_hash(),
            );
            self.arena.seal_inner_match(
                tournament.address,
                player_match.event.id,
                left,
                right,
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
        let (left, right) = if let Some(children) = commitment.merkle.node_children(player_match.state.other_parent) {
            children
        } else {
            return Ok(())
        };

        let (new_left, new_right) = if left != player_match.state.left_node {
            commitment.merkle.node_children(left).expect("left node does not have children")
        } else {
            commitment.merkle.node_children(right).expect("right node does not have children")
        };

        info!(
            "advance match with current height {} in tournament {} of level {} for commitment {}",
            player_match.state.current_height,
            player_match.tournament,
            self.tournament(player_match.tournament).level,
            commitment.merkle.root_hash(),
        );
        self.arena.advance_match(
            player_match.tournament,
            player_match.event.id,
            left,
            right,
            new_left,
            new_right,
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

    fn tournament(&self, address: Address) -> Arc<PlayerTournament> {
        self.tournaments.iter()
            .find(|t| t.address == address)
            .expect("tournament not found")
            .clone()
    }
}