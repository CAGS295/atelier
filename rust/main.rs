pub mod data;
pub mod simulation;

use crate::data::data_types::{Orderbook, Level, Order, Bids, Asks};
use crate::simulation::randomizer::randomize_order;

use std::collections::VecDeque;

fn main() {
    
    let mut ob = Orderbook {
        id: 1234,
        ts: 1234.1234,
        symbol: String::from("BTCUSDT"),
        bids: Bids { levels: Vec::new() },
        asks: Asks { levels: Vec::new() },
    };

    for _ in 0..10 {
        
        let (ts, price, amount, is_buy) = randomize_order();
        
        let order = Order {
            id: 123,
            ts,
            price,
            amount,
            is_buy,};

        if is_buy {
            ob.bids.levels.push(
                Level { 
                    id: 123,
                    price,
                    orders: VecDeque::from_iter(vec![order]).into(),
                }
            );
        } else {
            ob.asks.levels.push(
                Level {
                    id: 123,
                    price,
                    orders: VecDeque::from_iter(vec![order]).into(),
                }
            );
        }
    }
 
}
