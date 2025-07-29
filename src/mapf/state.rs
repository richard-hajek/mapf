use crate::deps::sparse::SparseMatrix2D;
use crate::mapf::definition::MAPFDefinition;
use crate::mapf::state::SpecialState::No;
use derive_more::Display;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone)]
pub struct MAPFState {
    pub definition: Arc<MAPFDefinition>,

    pub units_begin: SparseMatrix2D,
    pub units_available: SparseMatrix2D,
    pub units_moved: SparseMatrix2D,

    pub playing: u8,

    pub special_state: SpecialState
}

#[derive(Clone, PartialOrd, PartialEq, Hash, Debug, Display)]
pub enum SpecialState {
    No,
    AnyGoal,
}

impl PartialEq for MAPFState {
    fn eq(&self, other: &Self) -> bool {

        if self.special_state != No || other.special_state != No {
            return self.special_state == other.special_state;
        }

        self.units_available == other.units_available
            && self.units_moved == other.units_moved
            && self.units_begin == other.units_begin
            && self.playing == other.playing
    }
}

impl Eq for MAPFState {}

impl Hash for MAPFState {
    fn hash<H: Hasher>(&self, state: &mut H) {

        if self.special_state != No {
            self.special_state.hash(state);
            return;
        }

        self.units_available.hash(state);
        self.units_moved.hash(state);
        self.units_begin.hash(state);
        self.playing.hash(state);
    }
}

impl Debug for MAPFState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        f.debug_struct("MAPFState")
            .field("playing", &self.playing)
            .field("stage0_positions", &self.units_begin.get_nnz())
            .field("stage1_positions", &self.units_available.get_nnz())
            .field("stage2_positions", &self.units_moved.get_nnz())
            .field("special_state", &self.special_state)
            .finish()
    }
}

impl fmt::Display for MAPFState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for a0_idx in 0..self.definition.shape.0 {
            for a1_idx in 0..self.definition.shape.1 {
                let obstacle = self.definition.obstacles.get(a0_idx, a1_idx).unwrap_or(0);
                let s0 = self.units_begin.get(a0_idx, a1_idx).unwrap_or(0);
                let s1 = self.units_available.get(a0_idx, a1_idx).unwrap_or(0);
                let s2 = self.units_moved.get(a0_idx, a1_idx).unwrap_or(0);
                let g = self.definition.goals.get(a0_idx, a1_idx).unwrap_or(0);
                let g_s = if g > 0 { (b'A' + (g - 1)) as char } else { ' ' };

                let bit_count_sum = s0.count_ones() + s1.count_ones() + s2.count_ones() + g.count_ones();

                if obstacle != 0 {
                    write!(f, "#    ")?;
                } else if s0 > 0 && (s0 == s1) && s2 == 0 {
                    write!(f, "{}    ", s0)?;
                } else if s0 > 0 && s1 > 0 && s2 > 0 && (s1 != s2) && s0 == s1{
                    write!(f, "⚔{}{} ", s1, s2)?;
                } else if bit_count_sum > 1 {
                    write!(f, "{}{}{}{} ", s0, s1, s2, g_s)?;
                } else if s0 != 0 {
                    write!(f, "<-   ")?;
                } else if s1 != 0 {
                    write!(f, "{}    ", self.units_available.get_checked(a0_idx, a1_idx))?;
                } else if s2 != 0 {
                    write!(f, "{}💨  ", self.units_moved.get_checked(a0_idx, a1_idx))?;
                } else if g != 0 {
                    write!(f, "{}    ", g_s)?;
                } else {
                    write!(f, ".    ")?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")?;

        Ok(())
    }
}