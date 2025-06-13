use rand::rngs::ThreadRng;
use crate::mapf::{MAPFAction, MAPFState};
use crate::state_definition::StateEnvironment;
use rand::seq::IndexedRandom;
use rand::rng;

pub trait AI<State, Action> {
    fn next(self: &mut Self, s: &State, e: &dyn StateEnvironment<State, Action>) -> Action where Self: Sized;
}

struct RandomMCTSAI {
    rng: ThreadRng,
}

impl RandomMCTSAI {
    fn new() -> RandomMCTSAI{
        RandomMCTSAI{
            rng: rng(),
        }
    }
}

impl AI<MAPFState, MAPFAction> for RandomMCTSAI {
    fn next(self: &mut Self, s: &MAPFState, e: &dyn StateEnvironment<MAPFState, MAPFAction>) -> MAPFAction
    where
        Self: Sized
    {
        let actions = e.get_actions(s);
        actions.choose(&mut self.rng).cloned().expect("No available actions")
    }
}