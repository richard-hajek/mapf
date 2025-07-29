use crate::ai::caboose_translation::limited_float::LimitedValue;
use crate::mapf::environment::MAPFEnvironment;
use crate::mapf::state::MAPFState;
use caboose::{Graph, GraphEdgeId, GraphNodeId, Move, State, TransitionSystem};
use std::collections::HashMap;
use std::slice::Iter;
use std::sync::Arc;
use tuple::{TupleElements, A2};

type SimpleNodeData = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SimpleState(pub GraphNodeId);
pub type SimpleEdgeData = f64;


pub struct MAPFEnvironmentCabooseCompat<'a> {
    source_environment: &'a MAPFEnvironment,
    source_state: &'a MAPFState,
    graph: Arc<Graph<SimpleNodeData, SimpleEdgeData>>
}

impl State for SimpleState {
    fn is_equivalent(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> MAPFEnvironmentCabooseCompat<'a> {
    pub fn new(s: &MAPFState, e: &MAPFEnvironment, playing_as: u8) -> MAPFEnvironmentCabooseCompat<'a>{
        let mut nodes_lookup: HashMap<(usize, usize), GraphNodeId> = HashMap::new();
        let mut node_positions = HashMap::new();
        let mut g: Graph<SimpleNodeData, SimpleEdgeData> = Graph::new();

        for i0 in 0..s.definition.shape.0 {
            for i1 in 0..s.definition.shape.1 {

                // obstacle is either obstacle or an enemy
                let obstacle = s.definition.obstacles.get(i0, i1).unwrap_or(0) != 0u8 || s.units_begin.get(i0, i1).unwrap_or(0) != playing_as;

                if obstacle {
                    let gid = g.add_node((i0, i1));
                    nodes_lookup.insert((i0, i1), gid);
                    node_positions.insert(gid, (i0, i1));
                }
            }
        }

        for i0 in 0..s.definition.shape.0 {
            for i1 in 0..s.definition.shape.1 {
                if let Some(from_id) = nodes_lookup.get(&(i0, i1)) {
                    if let Some(to_id) = nodes_lookup.get(&(i0 - 1, i1)) {
                        g.add_edge(*from_id, *to_id, 1f64);
                    }

                    if let Some(to_id) = nodes_lookup.get(&(i0, i1 - 1)) {
                        g.add_edge(*from_id, *to_id, 1f64);
                    }

                    if let Some(to_id) = nodes_lookup.get(&(i0 + 1, i1)) {
                        g.add_edge(*from_id, *to_id, 1f64);
                    }

                    if let Some(to_id) = nodes_lookup.get(&(i0, i1 + 1)) {
                        g.add_edge(*from_id, *to_id, 1f64);
                    }
                }
            }
        }

        MAPFEnvironmentCabooseCompat{
            source_environment: e,
            source_state: s,
            graph: Arc::new(g),
        }
    }
}

impl<'a> TransitionSystem<SimpleState, GraphEdgeId, LimitedValue<f64>, LimitedValue<f64>> for MAPFEnvironmentCabooseCompat<'a> {
    fn actions_from(&self, state: &SimpleState) -> Iter<GraphEdgeId> {
        self.graph.get_edges_out(state.0).iter()
    }

    fn transition(&self, _state: &SimpleState, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).to)
    }

    fn transition_cost(&self, state: &SimpleState, action: &GraphEdgeId) -> LimitedValue<f64> {
        1f64.into()
    }

    fn reverse_actions_from(&self, state: &SimpleState) -> Iter<GraphEdgeId> {
        self.graph.get_edges_in(state.0).iter()
    }

    fn reverse_transition(&self, _state: &SimpleState, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).from)
    }

    fn reverse_transition_cost(&self, state: &SimpleState, action: &GraphEdgeId) -> LimitedValue<f64> {
        1f64.into()
    }

    fn can_wait_at(&self, state: &SimpleState) -> bool {
        true
    }

    fn conflict(&self, moves: A2<&Move<SimpleState, GraphEdgeId, LimitedValue<f64>, LimitedValue<f64>>>) -> bool {
        let m0_to = moves.0.clone().to;
        let m1_to = moves.1.clone().to;
        m0_to == m1_to
    }
}