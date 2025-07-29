use std::hash::{Hash, Hasher};
use std::sync::Arc;
use crate::mapf::action::MAPFAction;
use crate::mapf::state::MAPFState;

#[derive(Clone, Debug)]
pub struct MAPFWrappedState {
    pub state: MAPFState,
    pub actions: Option<Arc<Vec<MAPFAction>>>
}

impl PartialEq for MAPFWrappedState{
    fn eq(&self, other: &Self) -> bool {
        self.state.eq(&other.state)
    }
}

impl Eq for MAPFWrappedState{}

impl Hash for MAPFWrappedState{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}