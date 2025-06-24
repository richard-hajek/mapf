use crate::ai::AI;
use crate::deps::state_definition::StateEnvironment;
use crate::mapf::action::MAPFAction;
use crate::mapf::environment::MAPFEnvironment;
use crate::mapf::state::MAPFState;
use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

pub struct RandomAI {
    rng: ThreadRng,
}

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI { rng: rng() }
    }
}

impl AI for RandomAI {
    fn next(self: &mut Self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction
    where
        Self: Sized,
    {
        let actions = e.get_actions(s);
        actions
            .choose(&mut self.rng)
            .cloned()
            .expect("No available actions")
    }
}