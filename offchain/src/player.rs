use std::{
    error::Error,
    collections::HashMap,
    rc::Rc,
};

use::log::info;

use crate::{
    arena::{Arena, Address, CommitmentProof, MatchState, MatchCreatedEvent},
    machine::{ComputationCommitment, Machine},
};

static LEVELS: u64 = 4;
static LOG2_STEP: [u64; 4] = [24, 14, 7, 0];
static LOG2_UARCH_SPAN: u64 = 16;
static UARCH_SPAN: u128 =  (1 << LOG2_UARCH_SPAN) - 1;
static HEIGHTS: [u64; 4] = [39, 10, 7, 7];



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
pub struct Player {
    arena: Box<dyn Arena>,
    machine: Box<dyn Machine>,
    root_tournament: Address,
    tournaments: Vec<Rc<PlayerTournament>>,
    matches: Vec<Rc<PlayerMatch>>,
    commitments: HashMap<Address, Rc<ComputationCommitment>>,
    called_win: HashMap<Address, bool>,
}

impl Player {
    pub fn new(arena: Box<dyn Arena>, machine: Box<dyn Machine>, root_tournamet: Address) -> Self {
        Self {
            arena: arena,
            machine: machine,
            root_tournament: root_tournamet,
            tournaments: vec![
                Rc::new(PlayerTournament{
                    address: root_tournamet,
                    level: LEVELS,
                    parent: None,
                    base_big_cycle: 0,
                }),
            ],
            matches: Vec::<Rc<PlayerMatch>>::new(),
            commitments: HashMap::<Address, Rc<ComputationCommitment>>::new(),
            called_win: HashMap::<Address, bool>::new(),
        }
    }

    pub async fn react(&mut self) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let last_tournament = self.tournaments.last().unwrap().clone();
        self.react_tournament(last_tournament).await
    }

    async fn react_tournament(
        &mut self,
        tournament: Rc<PlayerTournament>,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        let commitment = if let Some(commitment) = self.commitments.get(&tournament.address) {
            commitment.clone()
        } else {
            let commitment = self.machine.build_commitment(
                LOG2_STEP[tournament.level as usize],
                HEIGHTS[(LEVELS - tournament.level) as usize],
                false, 
                false,
            ).await?;
            self.commitments.insert(tournament.address, Rc::new(commitment));
            self.commitments.get(&tournament.address).unwrap().clone()
        };

        if tournament.parent.is_none() {
            if let Some((winner_commitment, _)) = self.arena.root_tournament_winner(tournament.address).await? {
                info!("tournament {} finished, winner is {}", tournament.address, winner_commitment);
                if commitment.root_hash == winner_commitment {
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
                if winner != old_commitment.root_hash {
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
                commitment.root_hash,
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
        commitment: Rc<ComputationCommitment>,
    ) -> Result<Option<Rc<PlayerMatch>>, Box<dyn Error>> {
        let matches = self.arena.created_matches(tournament, commitment.root_hash).await?;
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
        let step = 1 << LOG2_STEP[tournament.level as usize];
        let leaf_cycle = base + (step * match_state.running_leaf_position);
        let base_big_cycle = leaf_cycle >> LOG2_UARCH_SPAN;
        let player_match = Rc::new(PlayerMatch {
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
        tournament: Rc<PlayerTournament>,
        commitment: Rc<ComputationCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        let (clock, _) = self.arena.commitment(
            tournament.address,
            commitment.root_hash,
        ).await?;

        if clock.allowance != 0 {
            return Ok(())
        }

        let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root_hash) {
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
        player_match: Rc<PlayerMatch>,
        commitment: Rc<ComputationCommitment>,
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
        player_match: Rc<PlayerMatch>,
        commitment: Rc<ComputationCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        if player_match.state.level == 1 {
            let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root_hash) {
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
                return Ok(())
            }

            let cycle = player_match.state.running_leaf_position >> LOG2_UARCH_SPAN;
            let ucycle = player_match.state.running_leaf_position & UARCH_SPAN;
            let proof = self.machine.get_logs(cycle as u64, ucycle as u64).await?;
            
            self.arena.win_leaf_match(
                player_match.tournament,
                player_match.event.id,
                left_child,
                right_child,
                proof,
            ).await?;
        } else {
            self.new_tournament(player_match).await?;
        }
        
        Ok(())
    }

    async fn react_unsealed_match(
        &mut self,
        player_match: Rc<PlayerMatch>,
        commitment: Rc<ComputationCommitment>,
    ) -> Result<(), Box<dyn Error>> {        
        let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root_hash) {
            children
        } else {
            return Ok(())
        };

        let (initial_hash, initial_hash_proof) = if player_match.state.running_leaf_position == 0 {
            (commitment.implicit_hash, CommitmentProof::default())
        } else {
            commitment.prove_leaf(player_match.state.running_leaf_position)
        };

        let tournament = self.tournaments.iter().find(|t| t.address == player_match.tournament).unwrap();
        if tournament.level == 1 {
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left_child,
                right_child,
                initial_hash,
                initial_hash_proof
            ).await?;
        } else {
            self.arena.seal_leaf_match(
                tournament.address,
                player_match.event.id,
                left_child,
                right_child,
                initial_hash,
                initial_hash_proof
            ).await?;
            self.new_tournament(player_match).await?;
        }

        Ok(())
    }

    async fn react_running_match(
        &mut self,
        player_match: Rc<PlayerMatch>,
        commitment: Rc<ComputationCommitment>,
    ) -> Result<(), Box<dyn Error>> {
        let (left_child, right_child) = if let Some(children) = commitment.chidlren(commitment.root_hash) {
            children
        } else {
            return Ok(())
        };

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

        Ok(())
    }

    async fn new_tournament(
        &mut self,
        player_match: Rc<PlayerMatch>,
    ) -> Result<(), Box<dyn Error>> {
        let address = self.arena.created_tournament(
            player_match.tournament,
            player_match.event.id,
        ).await?.unwrap().new_tournament_address;

        let tournament = Rc::new(PlayerTournament {
            address: address,
            level: player_match.state.level - 1,
            parent: Some(player_match.tournament),
            base_big_cycle: player_match.leaf_cycle,
        });
        self.tournaments.push(tournament);

        Ok(())
    }
}