use crate::deps::sparse::SparseMatrix2D;


#[derive(Debug)]
pub struct MAPFDefinition {
    pub shape: (usize, usize),
    pub starting_positions: SparseMatrix2D,
    pub obstacles: SparseMatrix2D,
    pub goals: SparseMatrix2D,
    pub goals_num: [u64; 3],
}