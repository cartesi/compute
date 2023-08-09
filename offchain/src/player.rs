use std::{
    error::Error,
    collections::HashMap,
};

use::log::info;

use crate::{
    arena::{Arena, Address},
    machine::{ComputationCommitment, Machine},
};

static LEVELS: u64 = 4;
static LOG_2_STEP: [u64; 4] = [24, 14, 7, 0];
static HEIGHTS: [u64; 4] = [39, 10, 7, 7];

#[derive(Debug, Clone, Copy)]
pub enum PlayerTournamentResult {
    TournamentWon,
    TournamentLost,
}

#[derive(Debug, Clone, Copy)]
struct PlayerTournament {
    address: Address,
    level: u64,
    parent: Option<Address>,
}

#[derive(Debug, Clone, Copy)]
struct PlayerMatch {

}

pub struct Player {
    arena: Box<dyn Arena>,
    machine: Box<dyn Machine>,
    root_tournament: PlayerTournament,
    commitments: HashMap<Address, Box<ComputationCommitment>>,
    called_win: HashMap<Address, bool>,
}

impl Player {
    pub fn new(arena: Box<dyn Arena>, machine: Box<dyn Machine>, root_tournamet: Address) -> Self {
        Self {
            arena: arena,
            machine: machine,
            root_tournament: PlayerTournament { 
                address: root_tournamet,
                level: LEVELS,
                parent: None,
            },
            commitments: HashMap::<Address, Box<ComputationCommitment>>::new(),
            called_win: HashMap::<Address, bool>::new(),
        }
    }

    pub async fn react(&mut self) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        self.react_tournament(self.root_tournament).await
    }

    async fn react_tournament(
        &mut self,
        tournament: PlayerTournament
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
        
        let commitment = if let Some(commitment) = self.commitments.get(&tournament.address) {
            commitment
        } else {
            let commitment = self.machine.build_commitment(
                LOG_2_STEP[tournament.level],
                HEIGHTS[LEVELS - tournament.level],
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

        match self.latest_match(tournament, commitment).await? {
            Some(latest_match) => self.react_match(latest_match, commitment).await,
            None => {
                self.join_tournament_if_needed(tournament, commitment).await?;
                Ok(None)
            }
        }
    }

    async fn latest_match(
        &self,
        tournament: PlayerTournament,
        commitment: &ComputationCommitment,
    ) -> Result<Option<PlayerMatch>, Box<dyn Error>> {
    }

    async fn join_tournament_if_needed(
        &self,
        tournament: PlayerTournament,
        commitment: &ComputationCommitment,
    ) -> Result<(), Box<dyn Error>> {
    } 

    async fn react_match(
        &mut self,
        player_match: PlayerMatch,
        commitment: &ComputationCommitment,
    ) -> Result<Option<PlayerTournamentResult>, Box<dyn Error>> {
    }
}