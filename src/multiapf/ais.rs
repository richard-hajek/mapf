use rand::rngs::ThreadRng;
use crate::multiapf::mapf::{MAPFAction, MAPFEnvironment, MAPFState};
use rand::seq::IndexedRandom;
use rand::rng;
use crate::mapflib::state_definition::StateEnvironment;

pub trait AI {
    fn next(self: &mut Self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction;
}

pub struct RandomMCTSAI {
    rng: ThreadRng,
}

impl RandomMCTSAI {
    pub fn new() -> RandomMCTSAI{
        RandomMCTSAI{
            rng: rng(),
        }
    }
}

impl AI for RandomMCTSAI {
    fn next(self: &mut Self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction
    where
        Self: Sized
    {
        let actions = e.get_actions(s);
        actions.choose(&mut self.rng).cloned().expect("No available actions")
    }
}