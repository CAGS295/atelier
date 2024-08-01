
/// MarketDataTypes

#[derive(Debug)]
struct Bids {
    prices: Vec<f32>,
    volumes: Vec<f32>,
}

struct Asks {
    prices: Vec<f32>,
    volumes: Vec<f32>,
}

struct Orderbook {
    orderbookid: u16,
    symbol: String,
    timestamp: u32,
    bids: Vec<>,
    asks: Vec<>
}

