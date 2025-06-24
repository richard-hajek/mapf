use crate::mapf::action::MAPFAction;
use crate::mapf::environment::MAPFEnvironment;
use crate::mapf::state::MAPFState;

pub mod greedy;
pub mod mcts;
pub mod random_ai;

pub trait AI {
    fn next(self: &mut Self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction;
}