
// Orderbook data type and basic methods

use tch::Tensor;

#[derive(Debug)]
struct Bids {
    prices: Tensor,
    volumes: Tensor,
}

struct Asks {
    prices: Tensor,
    volumes: Tensor,

}

struct Orderbook{
    orderbookid: i32,
    symbol: String,
    timestamp: i32,
    bids: Bids,
    asks: Asks,
}

impl Orderbook {

    fn tob(&self) -> (Bids, Asks) {
        (self.Bids, self.Asks)
    }

    fn midprice (&self) -> u32 {
        self.Bids.prices[0] + self.Asks.prices[0] / 2.0 

    }

}

