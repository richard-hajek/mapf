#![feature(duration_constructors_lite)]
#![feature(unboxed_closures)]

use multiapf::ais::RandomAI;
use multiapf::loops::{args, evaluate_ai, EvaluateAIParams};
use multiapf::mapf::MAPFEnvironment;
use crate::multiapf::ais::GreedyAI;
use crate::multiapf::loops::{gather, GatherArgs};

mod mapflib;
mod multiapf;

fn main() {
    println!("Hello, world!");
    
    let env = MAPFEnvironment::new_from_file("./maps/defender.txt").unwrap();
    let r = evaluate_ai(&env, 
                        vec![Box::new(GreedyAI::new()), Box::new(RandomAI::new())], 
                        args!(EvaluateAIParams, verbose: true, max_iters: 200));

    // let many = || {
    //     evaluate_ai(&env, vec![Box::new(GreedyAI::new()), Box::new(RandomAI::new())], args!(EvaluateAIParams,))
    // };
    // 
    // let result = gather(many, args!(GatherArgs, loops: 20));

    println!("{:?}", r);
}
