#![feature(duration_constructors_lite)]
#![feature(unboxed_closures)]

use crate::ai::greedy::GreedyAI;
use crate::ai::random_ai::RandomAI;
use crate::loops::{args, evaluate_ai, EvaluateAIParams};
use crate::mapf::environment::MAPFEnvironment;

mod deps;
mod mapf;
mod ai;
pub mod loops;

fn main() {
    println!("Hello, world!");

    let env = MAPFEnvironment::new_from_file("./maps/defender.txt").unwrap();
    let r = evaluate_ai(&env, 
                        vec![Box::new(GreedyAI::new()), Box::new(RandomAI::new())], 
                        args!(EvaluateAIParams, verbose: true, max_iters: 200));
    println!("{:?}", r);
}
