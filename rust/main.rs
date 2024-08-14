pub mod simulation;
pub mod structs;
use crate::structs::marketdata::order::Order;
use crate::simulation::randomizer::randomize_order;

fn main() {
    println!("main.rs empty");
    let i_order = randomize_order();
    println!("{:?}", i_order);

}

