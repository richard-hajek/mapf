extern crate derive_more;

use crate::multiapf::ais::AI;
use crate::multiapf::mapf::{MAPFAction, MAPFEnvironment};
use std::error::Error;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use derive_more::Display;


#[derive(Debug, Display)]
pub struct TimeoutError {}

#[derive(Debug, Display)]
pub struct NotFinished {}

impl Error for TimeoutError{}
impl Error for NotFinished{}

macro_rules! args {
    ($ty:ty, $($field:ident : $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut params: $ty = Default::default();
        $(params.$field = $value;)*
        params
    }};
}
pub(crate) use args;
use crate::mapflib::state_definition::{StateEnvironment, StateStatus};


pub struct EvaluateAIParams {
    timeout: Duration,
    max_iters: u64,
}

impl Default for EvaluateAIParams {
    fn default() -> Self {
        EvaluateAIParams{
            timeout: Duration::from_mins(5),
            max_iters: 100_000,
        }
    }
}

pub fn evaluate_ai(mapf: &MAPFEnvironment, mut actors: Vec<Box<dyn AI>>, params: EvaluateAIParams) -> Result<u64, Box<dyn Error>>{
    let mut state = mapf.get_initial_state();

    let start = Instant::now();

    for iteration in 0..params.max_iters {
        let playing = state.playing;
        let action = actors[playing as usize].next(&state, &mapf);
        state = mapf.next(&state, &action);
        if iteration % 1000 == 0 {
            println!("Iteration {}", iteration);
        }

        if let MAPFAction::Commit = action {
            let status = mapf.get_status(&state);

            if let StateStatus::Winner(w) = status {
                return Ok(w);
            }
        }

        if Instant::now() - start > params.timeout {
            return Err(Box::new(TimeoutError{}));
        }
    }

    Err(Box::new(NotFinished{}))
}

pub struct GatherArgs {
    pub loops: u64,
}

impl Default for GatherArgs {
    fn default() -> Self {
        GatherArgs {
            loops: 100,
        }
    }
}

pub fn gather<F>(f: F, args: GatherArgs) -> Vec<Result<u64, Box<dyn Error>>>
where
    F: Fn() -> Result<u64, Box<dyn Error>>,
{
    let mut results: Vec<Result<u64, Box<dyn Error>>> = vec![];

    for _epoch in 0..args.loops {
        results.push(f());
    }

    results
}