use rand::seq::IndexedRandom;
use rand::rng;
use std::sync::Arc;
use crate::ais::AI;
use crate::mapf::{MAPFAction, MAPFEnvironment, MAPFState};
use crate::state_definition::StateEnvironment;
use crate::timing::Measurement;

pub fn dummy() {
    let problem = MAPFEnvironment::new();
    let mut rng = rng();

    const EPOCHS: u64 = 10;
    const ITERS: u64 = 1000;

    let mut m = Measurement::new(EPOCHS, ITERS);

    for epoch in 0..EPOCHS as usize {
        m.start_epoch();

        let mut state = problem.get_initial_state();

        for iteration in 0..ITERS {
            m.start_iter();
            
            let actions = problem.get_actions(&state);
            let action = actions.choose(&mut rng).unwrap();
            state = problem.next(&state, action);

            m.end_iter(epoch);

            if iteration % 1000 == 0 {
                println!("Epoch {}, Iteration {}", epoch, iteration);
            }
        }

        m.end_epoch();
        m.print_epoch_report(epoch);
    }

    m.print_report();
}

pub fn evaluate_ai(mapf: MAPFEnvironment, actors: Vec<Arc<dyn AI<MAPFState, MAPFAction>>>){

}