
use crate::structs::marketdata::order::Order;

pub struct Level {
    pub level_id: u32,
    pub side: bool,
    pub price: f64,
    pub orders: Vec<Order>,
}

impl Level {

   pub fn new(level_id:u32, side:bool, price:f64, orders:Vec<Order>) -> Self {
    Level {
            level_id,
            side,
            price,
            orders,
        }
    } 

}

