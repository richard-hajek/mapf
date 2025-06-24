pub const MOVES: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MAPFAction {
    Commit,
    Move((usize, usize), (usize, usize)),
}