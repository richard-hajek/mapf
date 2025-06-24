use crate::ai::greedy::GreedyAI;
use crate::ai::AI;
use crate::deps::state_definition::{StateEnvironment, StateStatus};
use crate::mapf::action::MAPFAction;
use crate::mapf::environment::MAPFEnvironment;
use crate::mapf::state::MAPFState;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};
use std::time::{Duration, Instant};

struct MCTSNode {
    state: MAPFState,
    parent: Option<usize>,
    children: Vec<(MAPFAction, usize)>,
    visits: u32,
    wins: f64,
    unexpanded_actions: Vec<MAPFAction>,
}

pub struct MCTSAI {
    rng: ThreadRng,
    exploration_weight: f64,
    simulation_limit: u32,
    time_limit: Duration,
    me: u8,
}

impl MCTSAI {
    pub fn new(us: u8, exploration_weight: f64, simulation_limit: u32, time_limit_ms: u64) -> Self {
        MCTSAI {
            me: us,
            rng: rng(),
            exploration_weight,
            simulation_limit,
            time_limit: Duration::from_millis(time_limit_ms),
        }
    }

    fn select_child(&self, node: &MCTSNode, nodes: &[MCTSNode]) -> usize {
        let mut best_score = f64::NEG_INFINITY;
        let mut best_child_index = 0;

        for (_, child_index) in &node.children {
            let child = &nodes[*child_index];

            let exploitation = child.wins / child.visits as f64;
            let exploration = (2.0 * (node.visits as f64).ln() / child.visits as f64).sqrt();
            let score = exploitation + self.exploration_weight * exploration;

            if score > best_score {
                best_score = score;
                best_child_index = *child_index;
            }
        }

        best_child_index
    }

    fn expand(
        &mut self,
        node_index: usize,
        nodes: &mut Vec<MCTSNode>,
        env: &MAPFEnvironment,
    ) -> Option<usize> {
        let node = &mut nodes[node_index];

        if node.unexpanded_actions.is_empty() {
            return None;
        }

        let action_index = self.rng.random_range(0..node.unexpanded_actions.len());
        let action = node.unexpanded_actions.remove(action_index);

        let new_state = env.next(&node.state, &action);

        let new_node_index = nodes.len();
        let new_node = MCTSNode {
            state: new_state.clone(),
            parent: Some(node_index),
            children: Vec::new(),
            visits: 0,
            wins: 0.0,
            unexpanded_actions: env.get_actions(&new_state).as_ref().clone(),
        };

        nodes.push(new_node);

        nodes[node_index].children.push((action, new_node_index));

        Some(new_node_index)
    }

    fn simulate(&mut self, state: &MAPFState, env: &MAPFEnvironment) -> f64 {
        let mut current_state = state.clone();
        let mut random_ai = GreedyAI::new();

        for _ in 0..self.simulation_limit {
            match env.get_status(&current_state) {
                StateStatus::Running => {
                    let action = random_ai.next(&current_state, env);
                    current_state = env.next(&current_state, &action);
                }
                StateStatus::Winner(winner) => {
                    return if winner == self.me as u64 { 1.0 } else { 0.0 };
                }
                StateStatus::Draw => {
                    return 0.5;
                }
            }
        }

        0.5
    }

    fn backpropagate(&self, node_index: usize, result: f64, nodes: &mut Vec<MCTSNode>) {
        let mut current_index = Some(node_index);

        while let Some(index) = current_index {
            let node = &mut nodes[index];
            node.visits += 1;
            node.wins += result;
            current_index = node.parent;
        }
    }

    fn best_action(&self, root_index: usize, nodes: &[MCTSNode]) -> MAPFAction {
        let root = &nodes[root_index];
        let mut best_visits = 0;
        let mut best_action = None;

        for (action, child_index) in &root.children {
            let child = &nodes[*child_index];
            if child.visits > best_visits {
                best_visits = child.visits;
                best_action = Some(action.clone());
            }
        }

        best_action.unwrap_or(MAPFAction::Commit)
    }
}

impl AI for MCTSAI {
    fn next(self: &mut Self, s: &MAPFState, e: &MAPFEnvironment) -> MAPFAction {
        let start_time = Instant::now();
        let mut nodes = Vec::new();

        let root_index = 0;
        nodes.push(MCTSNode {
            state: s.clone(),
            parent: None,
            children: Vec::new(),
            visits: 0,
            wins: 0.0,
            unexpanded_actions: e.get_actions(s).as_ref().clone(),
        });

        while start_time.elapsed() < self.time_limit {
            let mut current_index = root_index;

            while nodes[current_index].unexpanded_actions.is_empty() && !nodes[current_index].children.is_empty()
            {
                current_index = self.select_child(&nodes[current_index], &nodes);
            }

            let status = e.get_status(&nodes[current_index].state);

            if StateStatus::Running == status {
                current_index = self.expand(current_index, &mut nodes, e).unwrap_or(current_index);
            }

            let result = self.simulate(&nodes[current_index].state, e);

            self.backpropagate(current_index, result, &mut nodes);
        }

        self.best_action(root_index, &nodes)
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::mcts::MCTSAI;
    use crate::ai::random_ai::RandomAI;
    use crate::ai::AI;
    use crate::deps::state_definition::StateEnvironment;
    use crate::deps::state_definition::StateStatus::Running;
    use crate::mapf::action::MAPFAction;
    use crate::mapf::environment::MAPFEnvironment;

    #[test]
    fn test_ai_mcts() {
        let env = MAPFEnvironment::new_from_file("./maps/test_hall_run.test.txt").unwrap();
        let state = env.get_initial_state();
        let mut mcts_ai = MCTSAI::new(1, 1.414, 50, 100);
        let m = mcts_ai.next(&state, &env);
        assert!(matches!(m, MAPFAction::Move(_, _) | MAPFAction::Commit));
    }

    #[test]
    fn test_ai_correct() {
        let env = MAPFEnvironment::new_from_file("./maps/test_hall_run.test.txt").unwrap();
        let state = env.get_initial_state();
        let mut greedy_ai = MCTSAI::new(1, 1.414, 100, 1000);
        let m = greedy_ai.next(&state, &env);
        assert_eq!(m, MAPFAction::Move((1, 4), (1, 5)));
    }

    #[test]
    fn test_ai_full_game(){
        let env = MAPFEnvironment::new_from_file("./maps/test_hall_run.test.txt").unwrap();
        let mut state = env.get_initial_state();
        let mut mcts = MCTSAI::new(1, 1.414, 100, 1000);
        let mut random_ai = RandomAI::new();

        while env.get_status(&state) == Running {

            let a;

            match state.playing {
                1 => {a = mcts.next(&state, &env);}
                2 => {a = random_ai.next(&state, &env);}
                _ => {panic!("eh?")}
            }

            println!("Player {} ({}) did {:?}", state.playing, if state.playing == 1 {"mcts"} else {"random"}, a);
            state = env.next(&state, &a);

            println!("{}", state);
            println!();
        }

        println!("======Status======");
        println!("{:?}", env.get_status(&state));
        println!("{:?}", state);
        println!()
    }
}
