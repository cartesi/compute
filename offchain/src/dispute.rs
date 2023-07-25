use std::sync::Arc;

use ethers::prelude::abigen;
use ethers::providers::{Provider, Http};

abigen!(RootTournament, "src/abi/RootTournament.json");

#[derive(Debug, Default)]
pub struct Dispute {
}

impl Dispute {
    pub fn new() -> Self {
        Self{}
    }

    pub fn start() {
        
    }

    pub fn finish() {
    }
}