#![feature(duration_constructors_lite)]
#![feature(unboxed_closures)]

use multiapf::ais::RandomMCTSAI;
use multiapf::loops::{args, evaluate_ai, EvaluateAIParams};
use multiapf::mapf::MAPFEnvironment;
use crate::multiapf::loops::{gather, GatherArgs};

mod mapflib;
mod multiapf;

fn main() {
    println!("Hello, world!");
    
    let env = MAPFEnvironment::new_from_file("./maps/box.txt").unwrap();

    let many = || {
        evaluate_ai(&env, vec![Box::new(RandomMCTSAI::new()), Box::new(RandomMCTSAI::new())], args!(EvaluateAIParams,))
    };
    
    let result = gather(many, args!(GatherArgs, loops: 20));

    println!("{:?}", result);
}
