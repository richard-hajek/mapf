mod limited_float;
mod wrapped_state;
mod mapf_transition_environment;

use std::sync::Arc;
use std::time::Duration;
use crate::deps::state_definition::StateEnvironment;
use caboose::{CbsConfig, ConflictBasedSearch, Graph, GraphNodeId, Heuristic, Task, TransitionSystem};
use crate::ai::AI;
use crate::ai::caboose_translation::limited_float::LimitedValue;
use crate::ai::caboose_translation::mapf_transition_environment::{MAPFEnvironmentCabooseCompat, SimpleState};
use crate::mapf::action::MAPFAction;
use crate::mapf::environment::MAPFEnvironment;
use crate::mapf::state::MAPFState;

pub struct CabooseAI{
    player_id: u8,
}

impl CabooseAI {
    fn new(player_id: u8) -> CabooseAI{
        CabooseAI{
            player_id
        }
    }
}


impl AI for CabooseAI {
    fn next(&mut self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction {

        let env_compat = MAPFEnvironmentCabooseCompat::new(s, e, self.player_id);

        let tasks = vec![
            Arc::new(Task::new(
                SimpleState(GraphNodeId(0)),
                SimpleState(GraphNodeId(9)),
                0f64.into(),
            ))
        ];

        let env_compat_arc = Arc::new(env_compat);

        let config = CbsConfig::new(
            env_compat_arc.clone(),
            tasks,
            1e-6.into(),
            8,
            Some(Duration::from_secs(10))
        );

        let mut solver = ConflictBasedSearch::new(env_compat_arc.clone());
        let solutions = solver.solve(&config).unwrap();

        MAPFAction::Commit
    }
}



#[cfg(test)]
mod tests {
    use crate::ai::AI;
    use crate::ai::caboose_translation::CabooseAI;
    use crate::ai::caboose_translation::limited_float::LimitedValue;
    use crate::deps::state_definition::StateEnvironment;
    use crate::mapf::environment::MAPFEnvironment;

    #[test]
    fn test_01(){
        // let env = MAPFEnvironment::new_from_file("./maps/crossroads.txt").unwrap();
        // let mut ai = CabooseAI::new(0);
        // let initial = env.get_initial_state();
        // ai.next(&initial, &env);

        let v1: LimitedValue<f64> = LimitedValue::from(0f64);
        let v2: LimitedValue<f64> = LimitedValue::from(1000f64);

        let comp = v1 > v2;
        println!("{comp}");

        let v3 = v1 - v2;
        let v4 = v3 + v1;
    }

    #[test]
    fn test_02() {

        let env = MAPFEnvironment::new_from_file("./maps/box.txt").unwrap();
        let state = env.get_initial_state();
        let mut ai = CabooseAI::new(state.playing);
        ai.next(&state, &env);
    }

}