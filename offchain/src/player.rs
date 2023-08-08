use crate::arena::Arena;
use crate::machine::Machine;

pub struct Player {
    arena: Box<dyn Arena>,
    machine: Box<dyn Machine>,
}

impl Player {
    pub fn new(arena: Box<dyn Arena>, machine: Box<dyn Machine>) -> Self {
        Self {
            arena: arena,
            machine: machine,
        }
    }

    fn react_tournament(&mut self) {

    }

    fn react_match(&mut self) {

    }
}