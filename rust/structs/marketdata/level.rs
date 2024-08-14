
use crate::structs::marketdata::order::Order;

pub struct Level {
    pub level_id: u32,
    pub side: bool,
    pub price: f64,
    pub orders: Vec<Order>,
}

impl Level {

}

