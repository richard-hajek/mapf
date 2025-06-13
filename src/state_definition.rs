use std::sync::Arc;

pub trait StateEnvironment<S, A> {
    fn get_initial_state(&self) -> S;
    fn get_actions(&self, state: &S) -> Arc<Vec<A>>;

    fn next(&self, s: &S, a: &A) -> S;
    fn is_finished(&self, s: &S) -> bool;
    fn get_heuristic(&self) -> f64;
}