use crate::ai::AI;
use crate::deps::state_definition::StateEnvironment;
use crate::mapf::action::MAPFAction::Move;
use crate::mapf::action::{MAPFAction, MOVES};
use crate::mapf::environment::MAPFEnvironment;
use crate::mapf::state::MAPFState;
use std::collections::VecDeque;
use std::sync::Arc;

pub struct GreedyAI {
    grid: Arc<Option<Vec<Vec<i64>>>>,
}

impl GreedyAI {
    pub fn new() -> GreedyAI {
        GreedyAI {
            grid: Arc::new(None),
        }
    }
}

impl AI for GreedyAI {
    fn next(self: &mut Self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction
    where
        Self: Sized,
    {
        if self.grid.is_none() {
            let mut distances: Vec<Vec<i64>> = vec![vec![-1; s.definition.shape.1]; s.definition.shape.0];
            let mut queue: VecDeque<(usize, usize, i64)> = VecDeque::new();

            for (i_a0, i_a1) in s.definition.goals.get_nnz_list() {
                let a0 = i_a0 as usize;
                let a1 = i_a1 as usize;
                distances[a0][a1] = 0;
                queue.push_back((a0, a1, 0));
            }

            while !queue.is_empty() {
                let (i_a0, i_a1, distance) = queue.pop_front().unwrap();

                for (da0, da1) in MOVES {
                    let na0_ = (da0) + (i_a0 as isize);
                    let na1_ = (da1) + (i_a1 as isize);

                    if na0_ < 0 || na0_ >= s.definition.shape.0 as isize {
                        continue;
                    }

                    if na1_ < 0 || na1_ >= s.definition.shape.1 as isize {
                        continue;
                    }

                    let na0 = na0_ as usize;
                    let na1 = na1_ as usize;

                    if s.definition
                        .obstacles
                        .get(na0, na1)
                        .unwrap_or(0)
                        != 0
                    {
                        continue;
                    }

                    if distances[na0][na1] == -1 {
                        distances[na0][na1] = distance + 1;
                        queue.push_back((na0, na1, distance + 1));
                    }
                }
            }

            self.grid = Arc::new(Some(distances));
        }

        let actions = e.get_actions(s);

        let mut best_move: Option<MAPFAction> = None;
        let mut best_distance: Option<i64> = None;

        let grid = self.grid.as_ref().as_ref().unwrap();

        for action in actions.as_ref() {
            if let Move((f0, f1), (t0, t1)) = action {
                let target_distance = grid[*t0][*t1];
                if best_distance.is_none() || best_distance.unwrap() > target_distance {
                    best_move = Some(Move((*f0, *f1), (*t0, *t1)));
                    best_distance = Some(target_distance);
                }
            }
        }

        if let Some(bm) = best_move {
            return bm;
        }

        MAPFAction::Commit
    }
}


#[cfg(test)]
mod tests {
    use crate::ai::greedy::GreedyAI;
    use crate::ai::AI;
    use crate::deps::state_definition::StateEnvironment;
    use crate::mapf::action::MAPFAction;
    use crate::mapf::environment::MAPFEnvironment;

    #[test]
    fn test_ai_greedy(){
        let env = MAPFEnvironment::new_from_file("./maps/test_hall.test.txt").unwrap();
        let state = env.get_initial_state();
        let mut greedy_ai = GreedyAI::new();
        let m = greedy_ai.next(&state, &env);
        assert_eq!(m, MAPFAction::Move((1, 4), (1, 5)));
    }
}