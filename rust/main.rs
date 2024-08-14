pub mod structs;
pub mod simulation;
use crate::simulation::randomizer;

fn main() {
    
    // generating Order
    let i_order = randomizer::randomize_order();
    println!("{:?}", i_order);

    // Placeholder for generating Levels<Order>
    
    // Placeholder for generating Orderbook<Level>

}

