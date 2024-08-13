
// Orderbook data type and basic methods

#![allow(unused_imports)]
#![allow(dead_code)]

pub struct Bids {
    pub prices: Vec<f32>,
    pub volumes: Vec<f32>,
}

#[derive(Debug)]
pub struct Asks {
    pub prices: Vec<f32>,
    pub volumes: Vec<f32>,
}

pub struct Orderbook{
    pub orderbookid: u16,
    pub symbol: String,
    pub timestamp: u32,
    pub bids: Bids,
    pub asks: Asks,
}

impl Orderbook {

    fn new(bids: Bids, asks: Asks) -> Orderbook {
        Orderbook {
            orderbookid: 1234, 
            symbol: String::from("BTCUSDT"),
            timestamp: 1234,
            bids,
            asks, 
        }
    }

    // Top-Of-the-Book : best bid (price,volume) and ask (volume, price)
    pub fn midprice(&self) -> f32 {
         (self.bids.prices[0] + self.asks.prices[0])/2.0
    }

}

