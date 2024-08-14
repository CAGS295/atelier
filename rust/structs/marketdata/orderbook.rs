
use crate::structs::marketdata::level::Level;

pub struct Orderbook{
    pub orderbook_id: u32,
    pub orderbook_ts: f64,
    pub symbol: String,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook{
    pub fn new(orderbook_id: u32, orderbook_ts: f64, symbol: String, bids: Vec<Level>, asks: Vec<Level>) -> Self {
        Orderbook {
            orderbook_id,
            orderbook_ts,
            symbol,
            bids,
            asks,
        }
    }
}
