
use crate::structs::marketdata::level::Level;

pub struct Orderbook{
    pub orderbook_id: u32,
    pub orderbook_ts: f64,
    pub symbol: String,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook{

}
