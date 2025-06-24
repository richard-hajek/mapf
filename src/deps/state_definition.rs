use std::sync::Arc;


#[derive(Debug, PartialEq, Eq)]
pub enum StateStatus {
    Running,
    Winner(u64),
    Draw,
}

pub(crate) trait StateEnvironment<S, A> {
    fn get_initial_state(&self) -> S;
    fn get_actions(&self, state: &S) -> Arc<Vec<A>>;
    fn next(&self, s: &S, a: &A) -> S;
    fn get_status(&self, s: &S) -> StateStatus;
}